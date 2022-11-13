



const ALPHABET:&str = "abcdefghijklmnopqrstuvwxyz ";


#[derive(Debug)]
struct Vigenere{
    alphabet:Vec<char>,
    text: Vec<String>,
    key:String
}

impl Vigenere{
    fn new(text:String,key:String)->Self{
        let alphabet = ALPHABET.chars().map(|x| x).collect::<Vec<char>>();

        let text_ch = text.chars().collect::<Vec<char>>();

        let mut text:Vec<String> = Vec::new();
        for i in text_ch.chunks(key.len()){
    
            let a = i.into_iter().collect::<String>();
            text.push(a);
        }
    

        Self { alphabet, text,key }
    }



    fn where_is(&self,letter: char)->usize{
        let mut n = usize::default();

        for alp in 0..self.alphabet.len(){
            if letter == self.alphabet[alp]{
                n =  alp;
            }
        }
        n
    }




    fn encrypt(&self){
        let mut encrypted = Vec::<char>::new();
        let n = self.alphabet.len() as isize;


        for text in self.text.iter(){

            for ch in 0..text.len() {
                
                

                let text_c = text.chars().nth(ch).unwrap();
                let key_c = self.key.chars().nth(ch).unwrap();


                let w_t_c = self.where_is( text_c) as isize; 
                let k_t_c = self.where_is( key_c) as isize;

                let number = (w_t_c + k_t_c) % n;

                
                
                if number.is_negative(){
                    encrypted.push(self.alphabet[(n+number) as usize]);
                }else{
                    encrypted.push(self.alphabet[number as usize])

                }


                
            }

        }
        
        println!("{:?}",encrypted.into_iter().collect::<String>());

    } 



}








fn main() {


    let text = "Hello World".to_string();
    let key = "That is my key".to_string();

    let v = Vigenere::new(text,key);

    v.encrypt();







}
