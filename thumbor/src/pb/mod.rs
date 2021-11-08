use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;

// 声明 abi.rs
mod abi; 
pub use abi::*;

// 构造函数
impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// ImageSpec转base64字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// base64字符串转ImageSpec。比如 s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// Filter转为字符串，后续提供给photon_rs
impl filter::Filter {
    pub fn to_str(self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

// 自定义SampleFilter 转换为 photon_rs 的 SamplingFilter
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

// 提供一些辅助函数，让创建一个 spec 的过程简单一些
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_watermark(20, 20);
        let spec3 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
