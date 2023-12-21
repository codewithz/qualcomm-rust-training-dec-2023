
use std::fs::{File,OpenOptions,remove_file,read_to_string};
use std::io::{Read,BufReader,BufRead,Write,BufWriter,Error};

// Used to declare the static variables -- static variables 
// have a static lifetime -- they live for entire duration of program

static TEXT : &str="Lorem ipsum dolor sit amet consectetur adipisicing elit. Maxime mollitia,
molestiae quas vel sint commodi repudiandae consequuntur voluptatum laborum
numquam blanditiis harum quisquam eius sed odit fugiat iusto fuga praesentium
optio, eaque rerum! Provident similique accusantium nemo autem. Veritatis
obcaecati tenetur iure eius earum ut molestias architecto voluptate aliquam
nihil, eveniet aliquid culpa officia aut! Impedit sit sunt quaerat, odit,
tenetur error, harum nesciunt ipsum debitis quas aliquid. Reprehenderit,
quia. Quo neque error repudiandae fuga? Ipsa laudantium molestias eos 
sapiente officiis modi at sunt excepturi expedita sint? Sed quibusdam
recusandae alias error harum maxime adipisci amet laborum. Perspiciatis 
minima nesciunt dolorem! Officiis iure rerum voluptates a cumque velit ";

fn read_contents_glob(path: &str) ->Result<String,Error>{
    // let mut file_txt=String::new();
    // let mut readme=File::open(path)?;
    // let mut byte_str=String::new();
    // readme.read_to_string(&mut file_txt).map(|_| file_txt)
    read_to_string(path)
}

fn read_contents_buffered(path: &str) -> Result<String,Error>{
    let mut file_txt=String::new();
    let mut readme=File::open(path)?;

    let buffer=BufReader::new(readme);

    for line in buffer.lines(){
        file_txt.push_str(line?.as_str());
    }

    Ok(file_txt)

}

fn write_str_glob(path:&str, to_write: &str) -> Result<(),Error>{
    // let mut writeme=OpenOptions::new().append(true).open(path);
    let mut writeme=File::create(path)?;

    write!(writeme, "{}",to_write).map(|_| ())
}

fn append_str(path: &str, to_append: &str) -> Result<(),Error> {
    let mut writeme=OpenOptions::new().append(true).open(path)?;

    write!(writeme," \\ Appending...\n\n {}",to_append).map(|_| ())
}

fn write_str_buffered(path: &str,to_write : &str) -> Result<(),Error>{
    let writeme=File::create(path)?;
    let mut buffered_writer=BufWriter::new(writeme);

    let temp_str=to_write.to_string();

    for line in temp_str.lines(){
        writeln!(buffered_writer,"{}",line)?;
    }

    Ok(())
}

fn create_then_remove(path: &str) -> Result<(),Error>{
    File::create(path)?;

    remove_file(path)
}

fn write_bytes_to_file(filename:  &str,content: &[u8]) -> std::io::Result<()>{
    let mut file=File::create(filename)?;

    // Write the bytes 
    file.write_all(content)?;

    Ok(())
}

fn read_bytes_from_file(filename:&str) -> std::io::Result<Vec<u8>>{

    let mut file=File::open(filename)?;

    let mut content= Vec::new();
    file.read_to_end(&mut content)?;

    Ok(content)
}

fn copy_paste_file() -> std::io::Result<()>{

    let source_path="E:\\rust\\abc.zip";
    let destination_path="E:\\rust\\abc-copy.zip";

    let mut source_file=File::open(source_path)?;

    let mut destination_file=File::create(destination_path)?;

    let mut buffer =Vec::new();

    source_file.read_to_end(&mut buffer)?;

    destination_file.write_all(&buffer)?;

    println!("File Copy is Success!!");

    Ok(())
}

fn main() {
        println!("--------------------- Reading in String -----------------------------");
    match read_contents_glob("readme.txt") {
        Ok(file_contents) => println!("Content from
                                 readme.txt \n\n {} \n\n",file_contents),
        Err(err) => eprintln!("Error reading content {:?}",err)
    }

    // Reading in Buffered Order
    println!("--------------------- Reading in Buffered Order -----------------------------");
    match  read_contents_buffered("readme.txt") {
        Ok(file_contents) => println!("Content from
                                 readme.txt \n\n {} \n\n",file_contents),
        Err(err) => eprintln!("Error reading content {:?}",err)
    }

    println!("--------------------- Writng contnet of a String to File -----------------------------");
    match  write_str_glob("writeme.txt",TEXT) {
        Ok(_) => println!("Content written successfully"),
        Err(err) => eprintln!("Error reading content {:?}",err)
    }
    
    println!("--------------------- Appending  contnet of a String to File -----------------------------");
    match  append_str("writeme.txt",TEXT.to_uppercase().as_str()) {
        Ok(_) => println!("Content written successfully"),
        Err(err) => eprintln!("Error reading content {:?}",err)
    }
   
   
    println!("--------------------- Buffered Writing -----------------------------");
    match  write_str_buffered("writeme1.txt",TEXT.to_lowercase().as_str()) {
        Ok(_) => println!("Content written successfully"),
        Err(err) => eprintln!("Error reading content {:?}",err)
    }

    println!("--------------------- Creating and Removing a file -----------------------------");
    match  create_then_remove("removeme.txt") {
        Ok(_) => println!("File Created and Deleted successfully"),
        Err(err) => eprintln!("Error in create/remove file {:?}",err)
    }


    println!("--------------------- Write in Bytes -----------------------------");
    match  write_bytes_to_file("example-one.txt",b"Hello Rust") {
        Ok(_) => println!("File Written in Bytes"),
        Err(err) => eprintln!("Error in writing file {:?}",err)
    }

     
    println!("--------------------- Reading in Bytes -----------------------------");
    match  read_bytes_from_file("example-one.txt") {
        Ok(file_contents) => {println!("Content from
                                 readme.txt \n\n {:?} \n\n",file_contents);
            let string_from_vector=match String::from_utf8(file_contents) {

                Ok(s)=>s,
                Err(e)=>{
                    eprint!("Error {:?}",e);
                    String::new()
                }
           
            };
             println!("Content of file in String: {}",string_from_vector) 


                                },
        Err(err) => eprintln!("Error reading content {:?}",err)
    }

    println!("--------------------- Copy Paste in Bytes -----------------------------");
    match  copy_paste_file() {
        Ok(_) => println!("Copy Paste in Bytes"),
        Err(err) => eprintln!("Error in writing file {:?}",err)
    }

    


    
}
