use std::error::Error;

type Err = Box<dyn Error>;

struct Task<'a, C: Copy, T: Copy> {
    name: &'a str,
    ctx: C,
    result: T,
}

impl<'a, C: Copy, T: Copy> Task<'a, C, T> {
    fn new<F>(name: &'a str, ctx: C, f: F) -> Result<Self, Err>
    where
        F: FnOnce(C) -> Result<T, Err>,
    {
        Ok(Self {
            name,
            ctx,
            result: f(ctx)?,
        })
    }

    fn then<F, U>(&self, name: &'a str, f: F) -> Result<Task<C, U>, Err>
    where
        F: FnOnce(C, T) -> Result<U, Err>,
        U: Copy,
    {
        Ok(Task {
            name,
            ctx: self.ctx,
            result: f(self.ctx, self.result)?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let s = String::from("foobar");
    let result = Task::new("foo", (), |_| {
        println!("bar");
        Ok(1)
    })?
    .then("baz", |_, i| Ok(i + 1))?
    .then("foo", |_, i| Ok(i + 1))?
    .then(&s, |_, i| Ok(i + 1))?
    .result;
    println!("{:?}", result);
    Ok(())
}
