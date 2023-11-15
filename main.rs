fn main() {
    /*

    // EBBENE SI! Posso usare if per determinare con alcune condizioni diversi assegnamenti alle variabili come riportato qua:

    let a = if <condition>{
        <truepart>
    }else {
        <falsepart>
    };

    // MA ANCHE CON MATCH!

    let a = match <condition> {
        Ok(num) if num > 1 => num, //da nota che posso comunque usare if per determinare con alcune ulteriori condizioni
        _ => 42; // _ Ã¨ il valore di default.
    };

    prova reale:

    let a = match "1".parse::<i32>() {
        Ok(num) if num > 1 => num,
        _ => 42,
    }

    println!("{}", a);

    Parliamo rapidamente di Tuple e strutture dati.

    struct Point { //struttura dati
        x: i32,
        y: i32,
    }

    fn main() {
        let (a, b, c) = (42, 666, 777); // tupla
        let Point { x, y } = Point { x: a, y: b}; // struttura dati

        println!("a={} b={} c={}", a, b, c);
        println!("x={} y={}", x, y);
    }

        a=42 b=666 c=777
        x=42 y=666
    */
}
