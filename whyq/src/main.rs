use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
};

use nanoserde::DeJson;

trait Jq {
    fn jq<T: DeJson>(&self, query: &str) -> Result<T, anyhow::Error>;
}

impl Jq for &str {
    fn jq<T: DeJson>(&self, query: &str) -> Result<T, anyhow::Error> {
        let mut cmd = Command::new("jq")
            .args([query])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin = cmd.stdin.take().unwrap();
        stdin.write_all(self.as_bytes()).unwrap();
        drop(stdin);

        let output = cmd.wait_with_output()?;
        Ok(T::deserialize_json(&String::from_utf8(output.stdout)?)?)
    }
}

impl Jq for String {
    fn jq<T: DeJson>(&self, query: &str) -> Result<T, anyhow::Error> {
        self.as_str().jq(query)
    }
}

fn main() -> Result<(), anyhow::Error> {
    dbg!("{\"hello\": \"world\"}".jq::<String>(".hello")?);
    dbg!("{\"hello\": \"world\"}".jq::<Vec<String>>("[.hello]")?);

    #[derive(Debug, DeJson)]
    struct Foo {
        foo: String,
        bar: String,
    }
    dbg!(
        r#"
            {"foo": "bar", "bar": "baz"}
        "#
        .jq::<Vec<Foo>>("[{foo: .bar, bar: .foo}]")?
    );

    dbg!(
        ureq::get("https://api.github.com/repos/letsencrypt/boulder/pulls")
            .call()?
            .body_mut()
            .read_to_string()?
            .jq::<Vec<String>>("[.[].title]")?
    );

    Ok(())
}
