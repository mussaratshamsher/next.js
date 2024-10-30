use swc_core::ecma::{
    ast::{Pass, Program},
    visit::Visit,
};

pub fn linter<V>(visitor: V) -> impl Pass
where
    V: Visit,
{
    Linter { visitor }
}

struct Linter<V>
where
    V: Visit,
{
    visitor: V,
}

impl<V> Pass for Linter<V>
where
    V: Visit,
{
    fn process(&mut self, program: &mut Program) {
        self.visitor.visit_program(program);
    }
}
