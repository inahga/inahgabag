use std::error::Error;

struct Task<'a, C>
where
    C: Copy,
{
    name: &'a str,
    ctx: C,
}

impl<'a, C: Copy> Task<'a, C> {
    fn new(name: &'a str, ctx: C) -> Self
    where
        C: Copy,
    {
        Self { name, ctx }
    }

    fn run<F, T>(&self, name: &'a str, f: F) -> Result<TaskResult<C, T>, Box<dyn Error>>
    where
        F: FnOnce(C) -> Result<T, Box<dyn Error>>,
        T: Copy,
    {
        let result = f(self.ctx)?;
        Ok(TaskResult {
            ctx: self.ctx,
            result,
            name,
        })
    }
}

struct TaskResult<'a, C, T>
where
    C: Copy,
    T: Copy,
{
    ctx: C,
    name: &'a str,
    result: T,
}

impl<'a, C: Copy, T: Copy> TaskResult<'a, C, T> {
    fn then<F, U>(&self, name: &'a str, f: F) -> Result<TaskResult<C, U>, Box<dyn Error>>
    where
        F: FnOnce(C, T) -> Result<U, Box<dyn Error>>,
        C: Copy,
        T: Copy,
        U: Copy,
    {
        let result = f(self.ctx, self.result)?;
        Ok(TaskResult {
            ctx: self.ctx,
            result,
            name,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = Task::new("foo", ())
        .run("bar", |_| {
            println!("bar");
            Ok(1)
        })?
        .then("baz", |_, i| Ok(i + 1))?
        .then("foo", |_, i| Ok(i + 1))?
        .result;
    println!("{:?}", result);
    Ok(())
}
