use std::{any::Any, error::Error};

type FnResult<C> = Result<C, Box<dyn Error>>;
type TaskResult<'a, C> = Result<Task<'a, C>, Box<dyn Error>>;

#[derive(serde::Serialize, serde::Deserialize)]
struct Task<'a, C: Copy> {
    name: &'a str,
    completed: bool,
    ctx: C,
}

impl<'a, C: Copy> Task<'a, C> {
    fn then<F>(&self, name: &'a str, f: F) -> TaskResult<'a, C>
    where
        F: FnOnce(C) -> FnResult<C>,
        C: Clone,
    {
        let result = f(self.ctx)?;
        Ok(Task::<'a> {
            name,
            completed: true,
            ctx: result,
        })
    }
}

fn run<F>(name: &str, f: F) -> TaskResult<()>
where
    F: FnOnce() -> FnResult<()>,
{
    f()?;
    Ok(Task {
        name,
        completed: true,
        ctx: (),
    })
}

fn run_with<F, C>(name: &str, ctx: C, f: F) -> TaskResult<C>
where
    F: FnOnce(C) -> FnResult<C>,
    C: Copy,
{
    let result = f(ctx)?;
    Ok(Task {
        name,
        completed: true,
        ctx: result,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // stateful task runner
    // run arbitrary functions, but they're reentrant
    // state needs to be serializable
    // run(|| foobar).run(|| foobar)

    run("foo", || {
        println!("foo");
        Ok(())
    })?;

    run("foo", || {
        println!("foo");
        Ok(())
    })?
    .then("bar", |_| {
        println!("bar");
        Ok(())
    })?;

    run("foo", || {
        println!("foo");
        Ok(())
    })?
    .then("bar", |_| {
        println!("bar");
        Ok(())
    })?
    .then("baz", |_| {
        println!("baz");
        Ok(())
    })?;

    let foo = run_with("1", 1, |mut ctx| {
        ctx += 1;
        println!("{:?}", ctx);
        Ok(ctx)
    })?
    .then("2", |mut ctx| {
        ctx += 2;
        println!("{:?}", ctx);
        Ok(ctx)
    })?;
    println!("{:?}", foo.ctx);

    Ok(())
}
