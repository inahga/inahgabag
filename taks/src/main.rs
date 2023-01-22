use std::{any::Any, error::Error};

type FnResult = Result<Box<dyn Any>, Box<dyn Error>>;
type TaskResult<'a> = Result<Task<'a>, Box<dyn Error>>;

struct Task<'a> {
    name: &'a str,
    result: Box<dyn Any>,
}

impl<'a> Task<'a> {
    fn then<F>(&self, name: &'a str, f: F) -> TaskResult
    where
        F: FnOnce() -> FnResult,
    {
        let result = f()?;
        Ok(Task::<'a> { name, result })
    }
}

fn run<F>(name: &str, f: F) -> TaskResult
where
    F: FnOnce() -> FnResult,
{
    let result = f()?;
    Ok(Task { name, result })
}

fn main() -> Result<(), Box<dyn Error>> {
    // stateful task runner
    // run arbitrary functions, but they're reentrant
    // state needs to be serializable
    // run(|| foobar).run(|| foobar)

    run("foo", || {
        println!("foo");
        Ok(Box::new(()))
    })?;

    run("foo", || {
        println!("foo");
        Ok(Box::new(()))
    })?
    .then("bar", || {
        println!("bar");
        Ok(Box::new(()))
    })?;

    Ok(())
}
