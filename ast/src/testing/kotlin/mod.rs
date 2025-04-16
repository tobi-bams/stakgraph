use crate::lang::graphs::{EdgeType, NodeType};
use crate::lang::{CallsMeta, Graph};
use crate::{lang::Lang, repo::Repo};
use anyhow::Result;
use std::str::FromStr;
use test_log::test;

pub async fn test_kotlin_generic<G: Graph>() -> Result<(), anyhow::Error> {
    let repo = Repo::new(
        "src/testing/kotlin",
        Lang::from_str("kotlin").unwrap(),
        false,
        Vec::new(),
        Vec::new(),
    )
    .unwrap();

    let graph = repo.build_graph_inner::<G>().await?;

    let (num_nodes, num_edges) = graph.get_graph_size();
    assert_eq!(num_nodes, 145, "Expected 145 nodes");
    assert_eq!(num_edges, 148, "Expected 148 edges");

    fn normalize_path(path: &str) -> String {
        path.replace("\\", "/")
    }

    let language_nodes = graph.find_nodes_by_type(NodeType::Language);
    assert_eq!(language_nodes.len(), 1, "Expected 1 language node");
    assert_eq!(
        language_nodes[0].name, "kotlin",
        "Language node name should be 'kotlin'"
    );
    assert_eq!(
        normalize_path(&language_nodes[0].file),
        "src/testing/kotlin/",
        "Language node file path is incorrect"
    );

    let build_gradle_files = graph.find_nodes_by_name(NodeType::File, "build.gradle.kts");
    assert_eq!(
        build_gradle_files.len(),
        2,
        "Expected 2 build.gradle.kts files"
    );
    assert_eq!(
        build_gradle_files[0].name, "build.gradle.kts",
        "Gradle file name is incorrect"
    );

    let libraries = graph.find_nodes_by_type(NodeType::Library);
    assert_eq!(libraries.len(), 44, "Expected 44 libraries");

    let imports = graph.find_nodes_by_type(NodeType::Import);
    assert_eq!(imports.len(), 9, "Expected 9 imports");

    let classes = graph.find_nodes_by_type(NodeType::Class);
    assert_eq!(classes.len(), 6, "Expected 6 classes");

    let mut sorted_classes = classes.clone();
    sorted_classes.sort_by(|a, b| a.name.cmp(&b.name));

    assert_eq!(
        sorted_classes[1].name, "ExampleInstrumentedTest",
        "Class name is incorrect"
    );
    assert_eq!(
        normalize_path(&sorted_classes[1].file),
        "src/testing/kotlin/app/src/androidTest/java/com/kotlintestapp/ExampleInstrumentedTest.kt",
        "Class file path is incorrect"
    );

    let functions = graph.find_nodes_by_type(NodeType::Function);
    assert_eq!(functions.len(), 45, "Expected 45 functions");

    let data_models = graph.find_nodes_by_type(NodeType::DataModel);
    assert_eq!(data_models.len(), 1, "Expected 1 data model");

    let requests = graph.find_nodes_by_type(NodeType::Request);
    assert_eq!(requests.len(), 6, "Expected 6 requests"); // FIXME: Records more than 2 requests

    let calls_edges_count = graph.count_edges_of_type(EdgeType::Calls(CallsMeta::default()));
    assert!(calls_edges_count > 0, "Expected at least one calls edge");

    Ok(())
}

#[test(tokio::test)]
async fn test_kotlin() {
    use crate::lang::graphs::ArrayGraph;
    test_kotlin_generic::<ArrayGraph>().await.unwrap();
}

// #[test(tokio::test)]
// async fn test_kotlin_btree() {
//     use crate::lang::graphs::BTreeMapGraph;
//     test_kotlin_generic::<BTreeMapGraph>().await.unwrap();
// }
