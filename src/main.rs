extern crate image;
use image::imageops;
use std::env;
use std::fs::File;
use std::io::Write;
use ini::Ini;


fn main() {
    let args: Vec<String> = env::args().collect();
    let dir: String;
    let size :u32;
    if args.len() > 2{
        dir = String::from(&args[1]);
        size = args[2].parse::<u32>().expect("입력을 받아들이는것을 실패했습니다.");
    } else {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("입력을 받아들이는 것을 실패했습니다.");
        dir = String::from(s.trim());
        s = String::new();
        std::io::stdin().read_line(&mut s).expect("입력을 받아들이는 것을 실패했습니다.");
        println!("{}",s);
        size = s.trim().parse::<u32>().expect("입력을 받아들이는것을 실패했습니다.");
    }
    
    let mut opt = match Ini::load_from_file("options.ini"){
        Ok(file) => file,
        Err(_) => {
            println!("설정을 불러오는것을 실패했습니다. 새로운 파일을 만듭니다.");
            opt_new()
        }
    };


    let img = image::open(dir)
        .expect("이미지를 여는것을 실패했습니다.")
        .to_luma8();
    img_to_ascii(img, size, &opt);
    opt.write_to_file("options.ini").unwrap();
}

fn opt_new() -> Ini{
    let mut opt = Ini::new();
    opt.with_section(Some("Output"))
        .set("location", "output.txt")
        .set("reverse", "False")
        .set("letterset", "COMPLEX")
        .set("colored", "False");
    return opt
}

fn img_to_ascii(img: image::GrayImage, size: u32, opt: &Ini){
    let SIMPLE = vec!["@","%","#","*","+","=","-",":","."," "];
    let COMPLEX = vec!["$","@","B","%","8","&","W","M","#","*","o","a","h","k","b","d","p","q","w","m","Z","O","0","Q","L","C","J","U","Y","X","z","c","v","u","n","x","r","j","f","t","/","\\","|","(",")","1","{","}","[","]","?","-","_","+","~","<",">","i","!","l","I",";",":",",","\"","^","`","'","."," "];
    let location = opt.get_from(Some("Output"),  "location").unwrap();
    let letterset = match opt.get_from(Some("Output"), "letterset").unwrap(){
        "SIMPLE" => SIMPLE,
        "COMPLEX" => COMPLEX,
        _ => panic!("ini에 문제가 있습니다")
    };
    let reverse = match opt.get_from(Some("Output"), "reverse").unwrap(){
        "True" => true,
        "False" => false,
        _ => panic!("ini에 문제가 있습니다")
    };
    let dimensions = img.dimensions();
    let ascii_width = size;
    let ascii_height = size*dimensions.1/dimensions.0/2;
    let patch_width = dimensions.0/ascii_width;
    let patch_height = dimensions.1/ascii_height;
    let mut file = File::create(location).expect("파일을 생성하는 것을 실패했습니다.");
    let length = letterset.len() as f32;

    let mut luma : f32;
    for j in 0..ascii_height{
        for i in 0..ascii_width{
            let patch = imageops::resize(
                &imageops::crop_imm(&img,dimensions.0*i/ascii_width,dimensions.1*j/ascii_height,patch_width,patch_height)
                    .to_image(),
                1,
                1,
                imageops::FilterType::Lanczos3);
            if reverse{
                luma = (patch.get_pixel(0,0)[0]) as f32/255.1;
            } else{
                luma = (255-patch.get_pixel(0,0)[0]) as f32/255.1
            }
            let text = letterset[(luma*length) as usize ];
            print!("{}",text);
            file.write_all(text.as_bytes()).expect("문자를 기록하는 것을 실패했습니다.");
        }
        println!("");
        file.write_all(b"\n").expect("문자를 기록하는 것을 실패했습니다.");
    }
}