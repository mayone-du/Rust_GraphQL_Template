use juniper::FieldResult;
use juniper::RootNode;

use juniper::{GraphQLInputObject, GraphQLObject};

// GraphQLで扱う構造体の定義
#[derive(GraphQLObject)]
#[graphql(description = "Hello struct")]
struct Hello {
  id: String,
  message: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "Hoge struct")]
struct Hoge {
  id: String,
  message: String,
  foo: String,
}

// GraphQLクエリの引数で受け取る構造体の定義
#[derive(GraphQLInputObject)]
#[graphql(description = "NewHello struct")]
struct NewHello {
  message: String,
}

// クエリの定義（Get用）
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  // humanというqueryの名前で実行できる 引数にString型のidを受け取り、Hello構造体を返す？
  fn human(id: String) -> FieldResult<Hello> {
    // ただの静的なメッセージを返す
    Ok(Hello {
      id: "0".to_owned(),
      message: "Hello GraphQL!".to_owned(),
    })
  }

  fn hoge(id: String) -> FieldResult<Hoge> {
    Ok(Hoge {
      id: id.to_owned(),
      message: "Hoge String".to_owned(),
      foo: String::from("foooooo"),
    })
  }
}

// ミューテーションの定義（Post用）
pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
  // createHello(newHello{message: "foo"})の形式でmutationを実行できる
  fn create_hello(new_hello: NewHello) -> FieldResult<Hello> {
    Ok(Hello {
      id: "1234".to_owned(),
      message: new_hello.message,
    })
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
