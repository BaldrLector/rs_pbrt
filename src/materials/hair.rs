//std
use std::sync::Arc;
// pbrt
use core::interaction::SurfaceInteraction;
use core::material::{Material, TransportMode};
use core::microfacet::TrowbridgeReitzDistribution;
use core::paramset::TextureParams;
use core::pbrt::{Float, Spectrum};
use core::reflection::{Bsdf, Bxdf, FresnelConductor, MicrofacetReflection};
use core::texture::Texture;
use textures::constant::ConstantTexture;

pub struct HairMaterial {
    pub sigma_a: Option<Arc<Texture<Spectrum> + Sync + Send>>,
    pub color: Option<Arc<Texture<Spectrum> + Sync + Send>>,
    pub eumelanin: Option<Arc<Texture<Float> + Sync + Send>>,
    pub pheomelanin: Option<Arc<Texture<Float> + Sync + Send>>,
    pub eta: Arc<Texture<Float> + Sync + Send>, // default: 1.55
    pub beta_m: Arc<Texture<Float> + Sync + Send>, // default: 0.3
    pub beta_n: Arc<Texture<Float> + Sync + Send>, // default: 0.3
    pub alpha: Arc<Texture<Float> + Sync + Send>, // default: 2.0
}

impl HairMaterial {
    pub fn new(
        sigma_a: Option<Arc<Texture<Spectrum> + Send + Sync>>,
        color: Option<Arc<Texture<Spectrum> + Send + Sync>>,
        eumelanin: Option<Arc<Texture<Float> + Send + Sync>>,
        pheomelanin: Option<Arc<Texture<Float> + Send + Sync>>,
        eta: Arc<Texture<Float> + Send + Sync>,
        beta_m: Arc<Texture<Float> + Send + Sync>,
        beta_n: Arc<Texture<Float> + Send + Sync>,
        alpha: Arc<Texture<Float> + Send + Sync>,
    ) -> Self {
        HairMaterial {
            sigma_a: sigma_a,
            color: color,
            eumelanin: eumelanin,
            pheomelanin: pheomelanin,
            eta: eta,
            beta_m: beta_m,
            beta_n: beta_n,
            alpha: alpha,
        }
    }
    pub fn create(mp: &mut TextureParams) -> Arc<Material + Send + Sync> {
        let mut sigma_a: Option<Arc<Texture<Spectrum> + Send + Sync>> =
            mp.get_spectrum_texture_or_null(String::from("sigma_a"));
        let color: Option<Arc<Texture<Spectrum> + Send + Sync>> =
            mp.get_spectrum_texture_or_null(String::from("color"));
        let eumelanin: Option<Arc<Texture<Float> + Send + Sync>> =
            mp.get_float_texture_or_null(String::from("eumelanin"));
        let pheomelanin: Option<Arc<Texture<Float> + Send + Sync>> =
            mp.get_float_texture_or_null(String::from("pheomelanin"));
        if let Some(_sigma_a) = sigma_a.clone() {
            if let Some(_color) = color.clone() {
                println!("WARNING: Ignoring \"color\" parameter since \"sigma_a\" was provided.");
            }
            if let Some(_eumelanin) = eumelanin.clone() {
                println!(
                    "WARNING: Ignoring \"eumelanin\" parameter since \"sigma_a\" was provided."
                );
            }
            if let Some(_pheomelanin) = pheomelanin.clone() {
                println!(
                    "WARNING: Ignoring \"pheomelanin\" parameter since \"sigma_a\" was provided."
                );
            }
        } else if let Some(_color) = color.clone() {
            if let Some(_sigma_a) = sigma_a.clone() {
                println!("WARNING: Ignoring \"sigma_a\" parameter since \"color\" was provided.");
            }
            if let Some(_eumelanin) = eumelanin.clone() {
                println!("WARNING: Ignoring \"eumelanin\" parameter since \"color\" was provided.");
            }
            if let Some(_pheomelanin) = pheomelanin.clone() {
                println!(
                    "WARNING: Ignoring \"pheomelanin\" parameter since \"color\" was provided."
                );
            }
        } else if let Some(_eumelanin) = eumelanin.clone() {
            if let Some(_sigma_a) = sigma_a.clone() {
                println!(
                    "WARNING: Ignoring \"sigma_a\" parameter since \"eumelanin\" was provided."
                );
            }
            if let Some(_color) = color.clone() {
                println!("WARNING: Ignoring \"color\" parameter since \"eumelanin\" was provided.");
            }
        } else if let Some(_pheomelanin) = pheomelanin.clone() {
            if let Some(_sigma_a) = sigma_a.clone() {
                println!(
                    "WARNING: Ignoring \"sigma_a\" parameter since \"pheomelanin\" was provided."
                );
            }
            if let Some(_color) = color.clone() {
                println!(
                    "WARNING: Ignoring \"color\" parameter since \"pheomelanin\" was provided."
                );
            }
        } else {
            // default: brown-ish hair.
            sigma_a = Some(Arc::new(ConstantTexture::new(
                HairBSDF::sigma_a_from_concentration(1.3 as Float, 0.0 as Float),
            )));
        }
        let eta = mp.get_float_texture(String::from("eta"), 1.55);
        let beta_m = mp.get_float_texture(String::from("beta_m"), 0.3);
        let beta_n = mp.get_float_texture(String::from("beta_n"), 0.3);
        let alpha = mp.get_float_texture(String::from("alpha"), 2.0);
        Arc::new(HairMaterial::new(
            sigma_a,
            color,
            eumelanin,
            pheomelanin,
            eta,
            beta_m,
            beta_n,
            alpha,
        ))
    }
    pub fn bsdf(&self, si: &SurfaceInteraction) -> Bsdf {
        let mut bxdfs: Vec<Arc<Bxdf + Send + Sync>> = Vec::new();
        // WORK
        Bsdf::new(si, 1.0, bxdfs)
    }
}

impl Material for HairMaterial {
    fn compute_scattering_functions(&self,
                                    si: &mut SurfaceInteraction,
                                    // arena: &mut Arena,
                                    _mode: TransportMode,
                                    _allow_multiple_lobes: bool) {
        si.bsdf = Some(Arc::new(self.bsdf(si)));
    }
}

pub struct HairBSDF {
}

impl HairBSDF {
    pub fn sigma_a_from_concentration(ce: Float, cp: Float) -> Spectrum {
        let mut sigma_a: [Float; 3] = [0.0 as Float; 3];
        let eumelanin_sigma_a: [Float; 3] = [0.419 as Float, 0.697 as Float, 1.37 as Float];
        let pheomelanin_sigma_a: [Float; 3] = [0.187 as Float, 0.4 as Float, 1.05 as Float];
        for i in 0..3 {
            sigma_a[i] = (ce * eumelanin_sigma_a[i] + cp * pheomelanin_sigma_a[i]);
        }
        Spectrum::from_rgb(&sigma_a)
    }
}