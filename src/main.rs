use arrow::array::{Array, ArrayRef, ListArray, StringArray, StructArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::error::Result;
use datafusion::prelude::*;
use std::sync::Arc;

// Import the to_csv module
mod to_csv;
use to_csv::write_to_csv;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new session context
    let ctx = SessionContext::new();

    // Register the Parquet file as a table (update the file path accordingly)
    ctx.register_parquet(
        "cities_data",
        "path/to/cities.parquet", // Replace with your actual file path
        ParquetReadOptions::default(),
    )
    .await?;

    // Read the data into a DataFrame
    let df = ctx.table("cities_data").await?;

    // Select necessary columns
    let df = df.select_columns(&["continent", "country"])?;

    // Collect the DataFrame into RecordBatches
    let batches = df.collect().await?;

    // Prepare vectors to hold the flattened data
    let mut continents = vec![];
    let mut country_names = vec![];
    let mut cities = vec![];

    // Iterate over each batch to flatten the data
    for batch in batches {
        let num_rows = batch.num_rows();

        // Get columns by name
        let continent_col = batch
            .column_by_name("continent")
            .expect("continent column not found");
        let country_col = batch
            .column_by_name("country")
            .expect("country column not found");

        // Downcast columns to appropriate types
        let continent_array = continent_col
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("Failed to downcast continent column");

        let country_array = country_col
            .as_any()
            .downcast_ref::<StructArray>()
            .expect("Failed to downcast country column");

        // Extract 'name' and 'city' from 'country'
        let country_name_array = country_array
            .column_by_name("name")
            .expect("name field not found in country")
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("Failed to downcast country.name column");

        let city_array = country_array
            .column_by_name("city")
            .expect("city field not found in country")
            .as_any()
            .downcast_ref::<ListArray>()
            .expect("Failed to downcast country.city column");

        // Iterate over each row
        for row in 0..num_rows {
            let continent = continent_array.value(row);
            let country_name = country_name_array.value(row);

            let city_list_ref = city_array.value(row);
            let city_list = city_list_ref
                .as_any()
                .downcast_ref::<StringArray>()
                .expect("Failed to downcast city list");

            // Iterate over each city
            for city_index in 0..city_list.len() {
                let city = city_list.value(city_index);

                // Push the data into vectors
                continents.push(continent.to_string());
                country_names.push(country_name.to_string());
                cities.push(city.to_string());
            }
        }
    }

    // Create Arrow arrays from the vectors
    let continent_field = Field::new("continent", DataType::Utf8, false);
    let country_field = Field::new("country_name", DataType::Utf8, false);
    let city_field = Field::new("city", DataType::Utf8, false);

    let schema = Arc::new(Schema::new(vec![
        continent_field.clone(),
        country_field.clone(),
        city_field.clone(),
    ]));

    let continent_array = Arc::new(StringArray::from(continents)) as ArrayRef;
    let country_array = Arc::new(StringArray::from(country_names)) as ArrayRef;
    let city_array = Arc::new(StringArray::from(cities)) as ArrayRef;

    let flattened_batch = RecordBatch::try_new(
        schema.clone(),
        vec![continent_array, country_array, city_array],
    )?;

    // Create a MemTable from the RecordBatch
    let mem_table = MemTable::try_new(schema, vec![vec![flattened_batch]])?;

    // Register the MemTable as a table
    ctx.register_table("flattened_data", Arc::new(mem_table))?;

    // Now, we can run SQL queries on the flattened data

    // Query 1: Get the number of countries per continent
    let countries_per_continent = r#"
        SELECT continent, COUNT(DISTINCT country_name) AS country_count
        FROM flattened_data
        GROUP BY continent
    "#;

    // Query 2: Get the number of cities per country
    let cities_per_country = r#"
        SELECT country_name, COUNT(city) AS city_count
        FROM flattened_data
        GROUP BY country_name
    "#;

    // Query 3: Get the average number of cities per continent
    let avg_cities_per_continent = r#"
        SELECT continent, AVG(city_count) AS avg_city_count
        FROM (
            SELECT continent, country_name, COUNT(city) AS city_count
            FROM flattened_data
            GROUP BY continent, country_name
        )
        GROUP BY continent
    "#;

    // Query 4: Get the total number of cities by continent
    let total_cities_by_continent = r#"
        SELECT continent, COUNT(city) AS total_cities
        FROM flattened_data
        GROUP BY continent
    "#;

    // Query 5: Get the top 5 countries with the most cities
    let top_countries_by_cities = r#"
        SELECT country_name, COUNT(city) AS city_count
        FROM flattened_data
        GROUP BY country_name
        ORDER BY city_count DESC
        LIMIT 5
    "#;

    // Write query results to CSV files using the write_to_csv function from to_csv.rs
    write_to_csv(
        &ctx,
        countries_per_continent,
        "output_directory",
        "countries_per_continent.csv",
    )
    .await?;
    write_to_csv(
        &ctx,
        cities_per_country,
        "output_directory",
        "cities_per_country.csv",
    )
    .await?;
    write_to_csv(
        &ctx,
        avg_cities_per_continent,
        "output_directory",
        "avg_cities_per_continent.csv",
    )
    .await?;
    write_to_csv(
        &ctx,
        total_cities_by_continent,
        "output_directory",
        "total_cities_by_continent.csv",
    )
    .await?;
    write_to_csv(
        &ctx,
        top_countries_by_cities,
        "output_directory",
        "top_countries_by_cities.csv",
    )
    .await?;

    Ok(())
}
