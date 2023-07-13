fn main() {
    vertofor();
}

fn vertofor() {
    let list = vec![1, 2, 3, 4, 5, 6, 7];

    for el in &list {
        if el % 2 == 0 {
            println!("{}", el);
        }
    }
}

fn variant1vector() {
    let mut list = vec![1, 2, 3, 4, 5, 4, 3, 4];
    list.push(9);//bu vectore bir sayi eklemek icin
    list.remove(2);//bu vectorun icinden bir veriyi silmek icin

    /*println!("{:?}", list);//butun elementleri ekrana yazdirmak icin kullanilmaktadir
    println!("{}", &list[0]);//bu bir element almak icin kullanilir
    println!("{:?}", list.get(3));//burada 2 tur veri ekrana yadirilmaktadir bir tanesi some bir tanesi none , eger get fonksiyorunun icine var olan bir indexi yazarsam ekrana o indexteki veri bastirilir eger var olmayan bir index yazilir ekrana none yazdirilir
    //Get ozelliginin amaci olmayan bir indexe ilsailmaya calisilirken hata vermek yerine o indexin var olup olmadigini varsa o indexe ait sayiyi ekrana bastirmaktadir*/

    match list.get(60) {
        Some(el) => {
            println!("Element with index 6 is {}", el);
        }
        None => {
            println!("Element doesnt exist!");
        }
    }
}

fn variant2vector() {
    let mut list2: Vec<f64> = Vec::new();

    list2.push(1.0);
    list2.push(2.2);
    list2.push(3.2);
    println!("{list2:?}");
}

