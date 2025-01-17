// Here we have one function that does
// one thing: Adds one to whatever number
// we pass to it.
fn add_one(n: u64) -> u64 {
    n + 1
}

#[cfg(test)]
mod tests {

    // lets pull our add_one function into scope
    use super::*;

    // now let's pull in our lab tools into scope
    // to test our function
    use laboratory::{describe, expect, LabResult, NullState, SuiteContext};

    // From Rust's perspective we will only define
    // one test, but inside this test we can define
    // however many tests we need.
    #[test]
    fn aaa() -> LabResult {
        // let's describe what our add_one() function will do.
        // The describe function takes a closure as its second
        // argument. And that closure also takes an argument which
        // we will call "suite". The argument is the suite's context
        // and it allows for extensive customizations. The context struct
        // comes with a method called it() and using this method we can
        // define a test.
        describe("add_one()", |suite: &mut SuiteContext<NullState>| {
            // when describing what it should do, feel free to be
            // as expressive as you would like.
            suite
                .it("should return 1 when passed 0", |_| {
                    // here we will use the default expect function
                    // that comes with laboratory.
                    // We expect the result of add_one(0) to equal 1
                    expect(add_one(0)).to_equal(1)
                })
                // just as a sanity check, let's add a second test
                .it("should return 2 when passed 1", |_| {
                    expect(add_one(1)).to_equal(2)
                });
        })
        .state(NullState)
        .milis()
        .run()
    }
}
