mod hexdump{
    use std::{io::{self, Read}, fs::File, fmt};

    const BYTE_PER_LINE_SIZE: usize = 16;
    pub struct HexDumper{
        
        file: File,
        hex_data: Vec<Vec<u8>>,
    }

    impl HexDumper{

        pub fn new(filename: &str) -> io::Result<Self> {

            let f = File::open(filename)?; 
            
            Ok(HexDumper { file: f, hex_data: Vec::new() })
        }

        #[allow(dead_code)]
        pub fn open(filename: &str) -> io::Result<Self> {

            HexDumper::new(filename)
        }

        pub fn load(&mut self) -> io::Result<()> {
            
            let mut buf: [u8; BYTE_PER_LINE_SIZE] = [0; BYTE_PER_LINE_SIZE];
            while let Ok(_) =  self.file.read_exact(&mut buf) {
                self.hex_data.push(buf.to_vec());
            }

            Ok(())
        }

        pub fn to_string(&self) -> String {

            let mut str = String::new();
            let mut positon = 0;
            for line in self.hex_data.iter()  {

                let pos_str = format!("0x{:08X} ", positon);
                str.push_str(&pos_str);
                for byte in line.iter() {
                    let byte_str = format!("{:02X} ", *byte);
                    str.push_str(&byte_str);
                } 

               str.push_str("\n");
               positon += BYTE_PER_LINE_SIZE;
            }

            str
        }
    }

    impl fmt::Display for HexDumper {

       fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

            write!(f, "{}", self.to_string())
       } 
    }

}

// export HexDumper;
pub use hexdump::HexDumper;