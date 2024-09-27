## 日本語版

# データ解析プロジェクト

このプロジェクトでは、RustでDataFusionライブラリを使用してParquetファイルからデータを読み込み、データ解析を行い、その結果をCSVファイルとして出力します。

## 目次

- [必要条件](#必要条件)
- [セットアップ手順](#セットアップ手順)
- [プロジェクトの構造](#プロジェクトの構造)
- [実行方法](#実行方法)
- [結果](#結果)
- [注意事項](#注意事項)
- [ライセンス](#ライセンス)

---

### 必要条件

- Rust（バージョン1.8以上）
- Cargo（Rustに付属）
- Parquetファイル（`cities.parquet`）
- 以下のクレート（`Cargo.toml`で指定）：
  - `arrow`
  - `csv`
  - `datafusion`
  - `tokio`

### セットアップ手順

1. **Rustのインストール**

   Rustがインストールされていない場合、公式サイトからインストールしてください。

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **プロジェクトディレクトリの作成**

   プロジェクトのディレクトリを作成し、そこに移動します。

   ```bash
   mkdir Parquet_to_csv_Rust
   cd Parquet_to_csv_Rust
   ```

3. **Cargoプロジェクトの初期化**

   ```bash
   cargo init
   ```

4. **`Cargo.toml`の編集**

   `Cargo.toml`に必要な依存関係を追加します。

   ```toml
   [dependencies]
   arrow = "53.0.0"
   csv = "1.3.0"
   datafusion = "42.0.0"
   tokio = {version="1.0", features=["rt-multi-thread"]}
   ```

5. **ソースコードの追加**

   `src` ディレクトリ内に `main.rs` と `to_csv.rs` を作成し、提供されたコードをそれぞれに貼り付けます。

6. **`cities.parquet` ファイルの配置**

   `cities.parquet` ファイルをプロジェクトのルートディレクトリ（またはコード内で指定したパス）に配置します。

### プロジェクトの構造

```
Parquet_to_csv_Rust/
├── Cargo.toml
├── Cargo.lock
├── cities.parquet
└── src
    ├── main.rs
    └── to_csv.rs
```

- **Cargo.toml**: プロジェクトの依存関係を管理します。
- **cities.parquet**: 分析対象のデータファイルです。
- **src/main.rs**: メインのRustコードです。データの読み込み、解析、結果の出力を行います。
- **src/to_csv.rs**: クエリ結果をCSVファイルに書き込むためのモジュールです。

### 実行方法

1. **依存関係のビルド**

   プロジェクトのルートディレクトリで以下を実行して、依存関係をビルドします。

   ```bash
   cargo build
   ```

2. **プログラムの実行**

   以下のコマンドでプログラムを実行します。

   ```bash
   cargo run
   ```

3. **結果の確認**

   `output_directory` ディレクトリ内に生成されたCSVファイルが出力されます。

### 結果

以下のCSVファイルが生成されます：

- **countries_per_continent.csv**: 大陸ごとの国の数
- **cities_per_country.csv**: 国ごとの都市の数
- **avg_cities_per_continent.csv**: 大陸ごとの平均都市数
- **total_cities_by_continent.csv**: 大陸ごとの総都市数
- **top_countries_by_cities.csv**: 都市数が最も多い国トップ5

### 注意事項

- **ファイルパスの確認**: `main.rs` 内の `"path/to/cities.parquet"` を実際のファイルパスに置き換えてください。
- **出力ディレクトリの変更**: 必要に応じて、出力先のディレクトリ `"output_directory"` を変更してください。
- **データ型の拡張**: `to_csv.rs` の `array_value_to_string` 関数で、必要に応じてサポートされるデータ型を追加してください。
- **依存関係のバージョン**: 使用するクレートのバージョンは、ご使用の環境に合わせて調整してください。

### ライセンス

このプロジェクトはMITライセンスの下で提供されています。

---

## English Version

# Data Analysis Project

In this project, we use the DataFusion library in Rust to read data from a Parquet file, perform data analysis, and output the results as CSV files.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup Instructions](#setup-instructions)
- [Project Structure](#project-structure)
- [How to Run](#how-to-run)
- [Results](#results)
- [Notes](#notes)
- [License](#license)

---

### Prerequisites

- Rust (version 1.56 or later)
- Cargo (comes with Rust)
- Parquet file (`cities.parquet`)
- The following crates (specified in `Cargo.toml`):
  - `arrow`
  - `csv`
  - `datafusion`
  - `tokio`

### Setup Instructions

1. **Install Rust**

   If Rust is not installed, please install it from the official website.

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Create Project Directory**

   Create a project directory and navigate into it.

   ```bash
   mkdir Parquet_to_csv_Rust
   cd Parquet_to_csv_Rust
   ```

3. **Initialize a Cargo Project**

   ```bash
   cargo init
   ```

4. **Edit `Cargo.toml`**

   Add the necessary dependencies to your `Cargo.toml`.

   ```toml
   [dependencies]
   arrow = "53.0.0"
   csv = "1.3.0"
   datafusion = "42.0.0"
   tokio = {version="1.0", features=["rt-multi-thread"]}
   ```

5. **Add Source Code**

   Create `main.rs` and `to_csv.rs` in the `src` directory, and paste the provided code into each file respectively.

6. **Place the `cities.parquet` File**

   Place the `cities.parquet` file in the project's root directory (or the path specified in your code).

### Project Structure

```
Parquet_to_csv_Rust/
├── Cargo.toml
├── Cargo.lock
├── cities.parquet
└── src
    ├── main.rs
    └── to_csv.rs
```

- **Cargo.toml**: Manages the project's dependencies.
- **cities.parquet**: The data file to be analyzed.
- **src/main.rs**: The main Rust code that reads data, performs analysis, and outputs results.
- **src/to_csv.rs**: A module to write query results to CSV files.

### How to Run

1. **Build Dependencies**

   In the project's root directory, run the following to build dependencies.

   ```bash
   cargo build
   ```

2. **Run the Program**

   Execute the program with:

   ```bash
   cargo run
   ```

3. **Check the Results**

   CSV files will be generated in the `output_directory`.

### Results

The following CSV files will be generated:

- **countries_per_continent.csv**: Number of countries per continent
- **cities_per_country.csv**: Number of cities per country
- **avg_cities_per_continent.csv**: Average number of cities per continent
- **total_cities_by_continent.csv**: Total number of cities per continent
- **top_countries_by_cities.csv**: Top 5 countries with the most cities

### Notes

- **Verify File Paths**: Replace `"path/to/cities.parquet"` in `main.rs` with the actual file path.
- **Change Output Directory**: Adjust the output directory `"output_directory"` as needed.
- **Extend Data Types**: Add support for additional data types in the `array_value_to_string` function in `to_csv.rs` if necessary.
- **Dependency Versions**: Adjust the crate versions in `Cargo.toml` according to your environment.

### License

This project is provided under the MIT License.
