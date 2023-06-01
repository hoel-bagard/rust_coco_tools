#![allow(clippy::needless_pass_by_value)]
use anyhow::{Context, Result};
use cocotools::errors::MaskError;
use cocotools::mask::utils::Area;
use numpy::ndarray::Array;
use numpy::ndarray::ShapeBuilder;
use numpy::IntoPyArray;
use numpy::PyArray2;
use numpy::PyReadonlyArray2;
use pyo3::prelude::*;
use pyo3::pyfunction;

use cocotools::coco::object_detection;
use cocotools::mask;
use cocotools::mask::conversions;

use crate::coco::PyPolygons;
use crate::errors::PyMaskError;

#[allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]
#[pymodule]
#[pyo3(name = "mask")]
pub fn py_mask(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_rle, m)?)?;
    m.add_function(wrap_pyfunction!(decode_coco_rle, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly_rs, m)?)?;
    m.add_function(wrap_pyfunction!(decode_poly, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_coco_rle, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons, m)?)?;
    m.add_function(wrap_pyfunction!(encode_to_polygons_rs, m)?)?;
    m.add_function(wrap_pyfunction!(area, m)?)?;
    m.add_function(wrap_pyfunction!(to_bbox, m)?)?;
    Ok(())
}

fn decode<T>(py: Python<'_>, encoded_mask: T) -> Result<&PyArray2<u8>, PyMaskError>
where
    mask::Mask: TryFrom<T>,
    <mask::Mask as TryFrom<T>>::Error: Into<PyMaskError>,
{
    match mask::Mask::try_from(encoded_mask) {
        Ok(mask) => Ok(mask.into_pyarray(py)),
        Err(error) => Err(error.into()),
    }
}

#[pyfunction]
fn decode_rle(py: Python<'_>, encoded_mask: object_detection::Rle) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::Rle(encoded_mask),
    )?)
}

#[pyfunction]
fn decode_coco_rle(
    py: Python<'_>,
    encoded_mask: object_detection::CocoRle,
) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::CocoRle(encoded_mask),
    )?)
}

#[pyfunction]
fn decode_poly_rs(
    py: Python<'_>,
    encoded_mask: object_detection::PolygonsRS,
) -> PyResult<&PyArray2<u8>> {
    Ok(decode(
        py,
        &object_detection::Segmentation::PolygonsRS(encoded_mask),
    )?)
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
fn decode_poly(
    py: Python<'_>,
    poly: object_detection::Polygons,
    width: u32,
    height: u32,
) -> Result<&PyArray2<u8>, PyMaskError> {
    let mask = conversions::mask_from_poly(&poly, width, height).map_err(PyMaskError::from)?;
    Ok(mask.into_pyarray(py))
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_rle(mask: PyReadonlyArray2<u8>) -> object_detection::Rle {
    object_detection::Rle::from(&mask.to_owned_array())
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_coco_rle(mask: PyReadonlyArray2<u8>) -> PyResult<object_detection::CocoRle> {
    Ok(object_detection::CocoRle::try_from(&mask.to_owned_array()).map_err(PyMaskError::from)?)
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_polygons(mask: PyReadonlyArray2<u8>) -> PyPolygons {
    PyPolygons(conversions::poly_from_mask(&mask.to_owned_array()))
}

#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
fn encode_to_polygons_rs(mask: PyReadonlyArray2<u8>) -> object_detection::PolygonsRS {
    object_detection::PolygonsRS::from(&mask.to_owned_array())
}

#[pyfunction]
fn area(segmentation: object_detection::Segmentation) -> u32 {
    (&segmentation).area()
}

#[pyfunction]
fn to_bbox(segmentation: object_detection::Segmentation) -> object_detection::Bbox {
    object_detection::Bbox::from(&segmentation)
}
