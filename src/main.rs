//=== TUGAS AKHIR CAPSTONE WORKSHOP RUST TETI == 
// NAMA  : ALI AR RAMADHANI
// PRODI : TEKNIK FISIKA

use std::io;
use std::collections::HashMap;
//Mendeklarasikan modul yang akan dipakai

struct Profil{
   nama:String,
   inventory:i32, 
}
//Struct yang berisikan field String dan Integer

impl Profil {
     fn printnama(&mut self)->&str{
        &self.nama
     }
     fn printjumlahitem(&mut self)->i32{
        self.inventory
     }
}
//Membuat method untuk struct Profil

fn main(){
   let mut inventory = HashMap::new();
   //Mendefinisikan sebuah Hashmap yang akan berisikan key dan value untuk program management inventory ini
   //Key berupa string dan value berupa integer
   let controlvar = 1;
   let mut jumlah_item = 0;
   let mut inputnama = String::new();
   //Mendefinisikan variabel yang akan dipakai dalam program ini
   println!("Nama User : ");
   io::stdin().read_line(&mut inputnama).unwrap();
   //Mengambil input dari user untuk nama
   inputnama = inputnama.trim().to_string();
   while controlvar > 0 {
         //Masuk ke loop selama variabel tersebut memenuhi syarat
         let mut profil1 = Profil{nama : inputnama.clone(), inventory : jumlah_item};
         let mut input1 = String::new();
         clearscreen::clear().unwrap();
         //Baris ini untuk membersihkan terminal
         //Baris ini memerlukan dependencies clearscreen
         println!("Inventory Box Management");
         println!("User          : {}",profil1.printnama());
         println!("Item Quantity : {}",profil1.printjumlahitem());
         println!("------------------------");
         println!("1.Add Item");
         println!("2.Remove Item");
         println!("3.Check Inventory");
         println!("4.Close Manager");
         println!("Choose : ");
         //Output untuk Tampilan menu
         io::stdin().read_line(&mut input1).unwrap();
         input1 = input1.trim().to_string();
         if input1 == "1"{
            //Masuk ke opsi pertama untuk menambahkan entitas item pada Hashmap
            clearscreen::clear().unwrap();
            let mut inputkey = String::new();
            let mut inputvalue = String::new();
            println!("Storing Item");
            println!("------------------------");
            println!("Enter the item name : ");
            io::stdin().read_line(&mut inputkey).unwrap();
            //Input Nama Item, contoh : Milk,Bread,Corn,Flour etc
            inputkey = inputkey.trim().to_string();
            println!("Enter the item quantity : ");
            io::stdin().read_line(&mut inputvalue).unwrap();
            //Input jumlah item, hanya menerima integer dan float
            let trimmed = inputvalue.trim();
            match trimmed.parse::<i32>() {
               //Pada bagian ini dilakukan ErrorHandling agar jika user menginput float atau string akan
               //kembali ke menu utama
            Ok(_) => {
               
               let mut controlvarx = String::new();
               let inputvalueint:i32 = inputvalue.trim().parse().expect("Wrong Input");
            
            if inputvalueint <= 0{
                  println!("Wrong Input! Try Again");
                  io::stdin().read_line(&mut controlvarx).unwrap();
               }
               else if inputvalueint >=0{
               inventory.insert(inputkey,inputvalueint);
               jumlah_item += inputvalueint;}
               //Jika berhasil maka jumlah item akan ditambah dan item akan dimasukkan pada HashMap

            },
            Err(..) => {
                let mut controlvarx = String::new();
                println!("Wrong Input! Try Again");
                io::stdin().read_line(&mut controlvarx).unwrap();
                //Jika tidak maka akan diberi peringatan dan kembali ke menu utama
            }
         };

         }
         else if input1 == "2"{
            //Masuk ke opsi ke-2 untuk melakukan penghapusan/pembuangan suatu item
            clearscreen::clear().unwrap();
            let mut inputkey = String::new();
            println!("Disposing Item");
            println!("------------------------");
            println!("Enter the item name : ");
            io::stdin().read_line(&mut inputkey).unwrap();
            inputkey = inputkey.trim().to_string();
            let buang_item = match inventory.get(&inputkey) {
               Some(&v) => v, 
               None => {
                   println!("Tidak ada item bernama '{}' ", inputkey);
                   return;
                   //Bagian ini untuk ErrorHandling jika salah memasukkan nama item yang ingin dibuang
               },
           };

            jumlah_item = jumlah_item - buang_item;
            //Jumlah item akan dikurangi sesuai dengan jumlah(value) dari item(key) yang dibuang oleh user
            inventory.remove(&inputkey);

  

         }
         else if input1 == "3"{
            let mut controlvar2 = String::new();
            clearscreen::clear().unwrap();
            println!("  Item   |  Quantity");
            println!("_________|__________");
            for (key,value) in inventory.iter(){
                println!("-{:<6}  |  {}",key,value);
            }
            //Melakukan penampilan untuk setiap item yang ada di HashMap
            //Dilakukan dengan formatting agar rapi
            io::stdin().read_line(&mut controlvar2).unwrap();
            
         }
         else if input1 == "4"{
            clearscreen::clear().unwrap();
            //Pembersihan Terminal 
            println!("Closing Your Inventory....");
            break
            //Keluar dari loop, dengan begini Program akan diakhiri
            
         }
   }
}