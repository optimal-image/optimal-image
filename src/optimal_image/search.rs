use dataclients::*;
use image::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
extern crate dssim;
use EncodingConfig;
use ImageConfig;
use ImageFormat;
use ImageScore;

#[derive(Debug)]
pub struct SearchOptions {
    pub threshold: f64,
    pub quality_range: (u8, u8),
    pub formats: Vec<ImageFormat>,
}

pub type SearchResult = HashMap<ImageFormat, ImageConfig>;

pub struct Search {
    path: String,
    image_data: ImageData,
    options: SearchOptions,
    result: Option<SearchResult>,
    dssim: dssim::Dssim,
}

impl Search {
    pub fn from_path(
        path: &Path,
        options: SearchOptions,
    ) -> Result<Search, Box<Error>> {
        // TODO: initialize struct members based on path
        let extension = path.extension().unwrap().to_str();

        let original;
        if extension == Some("jpg") {
            let dataclient = Jpeg::new();
            original = dataclient.load(path)?;
        } else if extension == Some("png") {
            let dataclient = Png::new();
            original = dataclient.load(path)?;
        } else {
            return Err(Box::new(ImageError::FormatError(String::from(
                "error",
            ))));
        }

        let search = Search {
            path: path.to_str().unwrap().to_string(),
            image_data: original,
            options: options,
            result: None,
            dssim: dssim::new(),
        };

        Ok(search)
    }

    pub fn run(&mut self) -> Option<SearchResult> {
        // let mut config_map: HashMap<&ImageFormat, Vec<ImageConfig>> = HashMap::new();
        let configs = self.options.formats.iter().fold(
            HashMap::new(),
            |mut map: HashMap<&ImageFormat, Vec<ImageConfig>>, format| {
                let conf_list = (self.options.quality_range.0
                    ..self.options.quality_range.1)
                    .map(|q| {
                        let fmt = format.clone();
                        ImageConfig {
                            id: self.path.clone(),
                            encoding_config: EncodingConfig {
                                format: fmt,
                                quality: q,
                            },
                        }
                    })
                    .collect();

                map.insert(format, conf_list);
                map
            },
        );

        let result: SearchResult = configs.iter().fold(
            HashMap::new(),
            |mut result: SearchResult, (&k, v)| {
                let original =
                    self.dssim.create_image(&self.image_data).unwrap();
                let optimal_config = find_optimal_config(
                    &self.dssim,
                    &original,
                    &v,
                    self.options.threshold,
                );
                result.insert(k.clone(), optimal_config.clone());
                result
            },
        );

        println!("{:?}", result);

        self.result = Some(result.clone());

        Some(result)
    }

    pub fn get_result(&mut self) -> &Option<SearchResult> {
        &self.result
    }
}

pub fn find_optimal_config<'a>(
    dssim_context: &dssim::Dssim,
    original: &dssim::DssimImage<f32>,
    config: &'a &Vec<ImageConfig>,
    threshold: f64,
) -> &'a ImageConfig {
    let index = config
        .binary_search_by(|probe| {
            // TODO:
            // generate image with `probe` config
            // run dssim against original
            // e.g dssim_context.compare(original, new_image)
            // return Ordering according to threshold
            unimplemented!();
        })
        .unwrap();

    &config[index]
}

// rank based on min size with score above threshold
pub fn rank_image_scores(
    scores: &Vec<ImageScore>,
    score_threshold: f64,
) -> Vec<ImageScore> {
    let mut result = scores
        .iter()
        .cloned()
        .filter(|score| score.score >= score_threshold)
        .collect::<Vec<ImageScore>>();

    result.sort_unstable_by(|a, b| a.size.cmp(&b.size));

    result
}
