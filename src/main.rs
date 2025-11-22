/*mod args;
use args::Args;
use image::{io::Reader, DynamicImage, GenericImageView, ImageFormat,imageops::FilterType::Triangle};
use std::{fs::File, io::BufReader, u8, vec};
#[derive(Debug)]
enum ImageDataErrors{
    DifferentImageFormats,
    BufferTooSmall,
}

#[derive(Debug)]
struct FloatingImage{
 width:u32,
 height:u32,
 data:Vec<u8>,
 name:String,
}

impl FloatingImage{ 
  fn new(width:u32,height:u32,name:String)->Self{
    let buffer_capacity=height*width*4;
    let buffer=Vec::with_capacity(buffer_capacity.try_into().unwrap());

    FloatingImage { width, height, data: buffer, name }

  }  
fn set_data(& mut self,data:Vec<u8>)->Result<(),ImageDataErrors>{
if data.len()>self.data.capacity(){
    return Err(ImageDataErrors::BufferTooSmall);
}
self.data=data;
Ok(())
}


}

fn main() ->Result<(),ImageDataErrors>{


//"C:\Users\janet\Downloads\racecar.png"
// "C:\Users\janet\Downloads\race-car-png-file.png"
    let args= Args::new();
    let(image_1,image_format_1)=find_image_from_path(args.image_1);
    let(image_2,image_format_2)=find_image_from_path(args.image_2);
   // println!("{:?}",args);
   if image_format_1!=image_format_2{
    return Err(ImageDataErrors::DifferentImageFormats);
   }

let (image_1,image_2)=standardise_size(image_1, image_2);
let mut output=FloatingImage::new(image_1.width(), image_1.height(),args.output);
let combined_data=combine_images(image_1,image_2);
output.set_data(combined_data)?;
image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8,image_format_1).unwrap();
   Ok(())
}
fn find_image_from_path(path:String)->(DynamicImage,ImageFormat){
let image_reader=Reader::open(path).unwrap();
let image_format=image_reader.format().unwrap();
let image=image_reader.decode().unwrap();
(image,image_format)
}


fn get_smallest_dimension(dim_1:(u32,u32),dim_2:(u32,u32))->(u32,u32){
    let pix_1=dim_1.0*dim_1.1;
    let pix_2=dim_2.0*dim_2.1;
    return  if pix_1<pix_2 {dim_1}else{dim_2};
}

fn standardise_size(image_1:DynamicImage,image_2:DynamicImage)->(DynamicImage,DynamicImage){
let(width,height)=get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
print!("width:{}, height{}",width,height);
if image_2.dimensions()==(width,height){
    (image_1.resize(width, height, Triangle),image_2)
} else{
    (image_1,image_2.resize(width, height, Triangle))
}
}

fn combine_images(image_1:DynamicImage,image_2:DynamicImage)->Vec<u8>{
let vec_1=image_1.to_rgb8().into_vec();
let vec_2=image_2.to_rgb8().into_vec();

alternalte_pixels(vec_1,vec_2)
}

fn alternalte_pixels(vec_1:Vec<u8>,vec_2:Vec<u8>)->Vec<u8>{
let mut combined_data=vec![0u8;vec_1.len()];
//comnied_data==vec[0,0,0,0] if vec_1.len()=4
let mut i=0;
while i<vec_1.len(){
    if i%8==0{
        combined_data.splice(i..=i+3, set_rgba(&vec_1,i,i+3));
    }else{
        combined_data.splice(i..=i+3, set_rgba(&vec_2,i,i+3));
    }
    i+=4;
}
combined_data
}

fn set_rgba(vec:&Vec<u8>,start:usize,end:usize)->Vec<u8>{
let mut rgba=Vec::new();
for i in start..=end{
    let val=match vec.get(i){
        Some(d)=>*d,
        None=>panic!("Index out of bounds")

    };
    rgba.push(val);
}
rgba
}
*/

mod args;
use args::Args;
use image::{
    io::Reader, 
    DynamicImage, 
    GenericImageView, 
    ImageFormat, 
    imageops::FilterType::Triangle, 
    ColorType,
};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    IoError(String),
    DecodeError(String),
}

#[derive(Debug)]
struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = (height * width * 4)
            .try_into()
            .expect("Image too large for buffer");

        FloatingImage {
            width,
            height,
            data: Vec::with_capacity(buffer_capacity),
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();

    let (image_1, format_1) = find_image_from_path(&args.image_1)?;
    let (image_2, format_2) = find_image_from_path(&args.image_2)?;

    if format_1 != format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        &output.name,
        &output.data,
        output.width,
        output.height,
        ColorType::Rgba8,
        format_1,
    )
    .map_err(|e| ImageDataErrors::IoError(e.to_string()))?;

    Ok(())
}

fn find_image_from_path(path: &str) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    let image_reader = Reader::open(path)
        .map_err(|e| ImageDataErrors::IoError(format!("Failed to open {}: {}", path, e)))?;

    let format = image_reader
        .format()
        .ok_or_else(|| ImageDataErrors::DecodeError(format!("Unknown format for {}", path)))?;

    let image = image_reader
        .decode()
        .map_err(|e| ImageDataErrors::DecodeError(format!("Failed to decode {}: {}", path, e)))?;

    Ok((image, format))
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix_2 {
        dim_1
    } else {
        dim_2
    }
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
    println!("Resizing to width: {}, height: {}", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];
    let mut i = 0;

    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data[i..i + 4].copy_from_slice(&vec_1[i..i + 4]);
        } else {
            combined_data[i..i + 4].copy_from_slice(&vec_2[i..i + 4]);
        }
        i += 4;
    }

    combined_data
}
