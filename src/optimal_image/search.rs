use std::cmp::Ordering;
use ImageConfig;
use ImageScore;

pub fn find_optimal_config(config: &Vec<ImageConfig>, threshold: f64) -> &ImageConfig {
    let index = config.binary_search_by(|probe| {
        // generate image with `probe` config
        // run dssim against original
        // return Ordering according to threshold
        unimplemented!();
    }).unwrap();

    &config[index]
}

// rank based on min size with score above threshold
pub fn rank_image_scores(scores: &Vec<ImageScore>, score_threshold: f64) -> Vec<ImageScore> {
    let mut result = scores
        .iter()
        .cloned()
        .filter(|score| score.score >= score_threshold)
        .collect::<Vec<ImageScore>>();

    result.sort_unstable_by(|a, b| a.size.cmp(&b.size));

    result
}
