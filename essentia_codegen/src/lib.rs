mod algorithm_generation;
mod module_generation;

use algorithm_generation::{GeneratedAlgorithm, generate_algorithm_module_file};
use essentia_core::essentia::Essentia;
use std::collections::HashMap;
use std::path::Path;

use crate::module_generation::category_module::generate_category_module_file;
use crate::module_generation::main_module::generate_main_module_file;

/// Groups generated algorithms by category
fn group_algorithms_by_category(generated_algorithms: &[GeneratedAlgorithm]) -> HashMap<String, Vec<String>> {
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    for result in generated_algorithms {
        categories
            .entry(result.category_module_name.clone())
            .or_default()
            .push(result.algorithm_module_name.clone());
    }
    categories
}

/// Generates module files for all categories
fn generate_category_modules(
    out_dir: &Path,
    categories: &HashMap<String, Vec<String>>,
) -> std::io::Result<Vec<String>> {
    let mut sorted_categories: Vec<String> = categories.keys().cloned().collect();
    sorted_categories.sort();

    for category in &sorted_categories {
        if let Some(algo_vec) = categories.get(category) {
            generate_category_module_file(out_dir, category, algo_vec)?;
        }
    }

    Ok(sorted_categories)
}

fn generate_module_files(
    out_dir: &Path,
    generated_algorithms: &[GeneratedAlgorithm],
) -> std::io::Result<()> {
    let categories = group_algorithms_by_category(generated_algorithms);
    let sorted_categories = generate_category_modules(out_dir, &categories)?;
    generate_main_module_file(out_dir, &sorted_categories)?;
    Ok(())
}

pub fn generate_code(out_dir: &Path) -> std::io::Result<()> {
    let essentia = Essentia::new();

    let results: Vec<GeneratedAlgorithm> = essentia
        .available_algorithms()
        .map(|algorithm_name| {
            let algorithm = essentia.create_algorithm(algorithm_name)
                .expect("Algorithm should be available since it was listed in available_algorithms");
            let introspection = algorithm.introspection();

            generate_algorithm_module_file(introspection, out_dir)
        })
        .collect::<std::io::Result<_>>()?;

    generate_module_files(out_dir, &results)?;

    Ok(())
}
