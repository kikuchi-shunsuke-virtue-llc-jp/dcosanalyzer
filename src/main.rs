use polars::prelude::*;

fn main() -> PolarsResult<()> {
    // コマンドライン引数の1つ目にファイル名が指定されるので、それを取得する
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    println!("file_path: {}", file_path);

    // dataframe に読み込む
    let df = CsvReadOptions::default()
                .with_has_header(true)
                .try_into_reader_with_file_path(Some(file_path.into()))?
                .finish();

    println!("{:?}", df);

    Ok(())
}
