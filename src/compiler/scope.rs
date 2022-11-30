use crate::ast::ty::Ty;

pub struct Entity {
    pub level: usize,
    pub name: String,
    pub ty: Ty,
}

pub struct Scope {
    level: usize,
    entities: Vec<Entity>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            level: 0,
            entities: vec![],
        }
    }

    pub fn begin(&mut self) {
        self.level += 1;
    }

    pub fn end(&mut self) {
        self.entities.retain(|e| e.level < self.level);
        self.level -= 1;
    }

    pub fn add(&mut self, name: String, ty: Ty) {
        self.entities.push(Entity {
            level: self.level,
            name,
            ty,
        });
    }

    pub fn get(&self, name: String) -> Option<&Entity> {
        self.entities.iter().rev().find(|e| e.name == name)
    }
}

#[test]
fn test() {
    let mut scope = Scope::new();

    scope.add(String::from("a"), Ty::TyInt64);
    scope.add(String::from("b"), Ty::TyFloat64);
    assert_eq!(scope.get(String::from("a")).unwrap().ty, Ty::TyInt64);
    assert_eq!(scope.get(String::from("b")).unwrap().ty, Ty::TyFloat64);

    scope.begin();
    scope.add(String::from("a"), Ty::TyInt32);
    assert_eq!(scope.get(String::from("a")).unwrap().ty, Ty::TyInt32);
    assert_eq!(scope.get(String::from("b")).unwrap().ty, Ty::TyFloat64);
    scope.end();

    assert_eq!(scope.get(String::from("a")).unwrap().ty, Ty::TyInt64);
}
