fn main() {

   let days = [
       "first",
       "second",
       "third",
       "fourth",
       "fifth",
       "sixth",
       "seventh",
       "eighth",
       "ninth",
       "tenth",
       "eleventh",
       "twelfth",
   ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 0..12 {
          println!("\nOn the {} day of Christmas, my true love gave to me:", days[day]);
          for gift in (0..=day).rev() {
              if day > 0 && gift == 0 {
                  println!(" and {}", gifts[gift]);
              } else {
                  println!(" {}", gifts[gift]);
              }
          }
    }
   
}

//Output
// On the first day of Christmas, my true love sent to me:
// a partridge in a pear tree.

// On the second day of Christmas, my true love sent to me:
// two turtle doves,
// and a partridge in a pear tree.

