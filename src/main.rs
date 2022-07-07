use text_io::read;
use std::{thread, time::{self}};

#[macro_use]
extern crate colour;

fn main() {
    println!("Hello, welcome to a quiz about fundamentals about product innovation (1ZV50!) \nThere is hope, hopefully, to pass the exam. \n Are you ready? (Y/N)");
    let startanswer :String = read!();
    if startanswer == "Y" {
        println!("okay lets start!");
    } else if startanswer == "y" {
        println!("okay lets start!");
    } else {
        println!("So you are not ready? \nSelf destruct in 10 seconds");
        let mut destructioncounter :u8 = 10;
        while destructioncounter >= 1 {
            println!("{}", destructioncounter);
            destructioncounter -= 1;
            thread::sleep(time::Duration::from_secs(1));
        }

        panic!("Woops you are not ready!");
    }


    let mut questionsvec :Vec<i32> = (1..60).collect();
    let mut answeringvec = questions(&mut questionsvec);
    
    if answeringvec.len() > 0 {
        println!("Do you want to practice your mistakes (Y/N)?");
        let endanswer :String = read!();
        if endanswer == "Y" {
            questions(&mut answeringvec);
        } else if endanswer == "y" {
            questions(&mut answeringvec);
        } else {
            println!("So you are not ready? \nSelf destruct in 10 seconds");
            let mut destructioncounter :u8 = 10;
            while destructioncounter >= 1 {
                println!("{}", destructioncounter);
                destructioncounter -= 1;
                thread::sleep(time::Duration::from_secs(1));
            }
            panic!("Woops you are not ready!");
        }
    }



    thread::sleep(time::Duration::from_secs(1));
    println!("Do you want to practice again? (Y/N)");
    let endanswer :String = read!();
    if endanswer == "Y" {
        main();
    } else if endanswer == "y" {
        main();
    } else {
        println!("So you are not ready? \nSelf destruct in 10 seconds");
        let mut destructioncounter :u8 = 10;
        while destructioncounter >= 1 {
            println!("{}", destructioncounter);
            destructioncounter -= 1;
            thread::sleep(time::Duration::from_secs(1));
        }
        panic!("Woops you are not ready!");
    }
}

fn anykey() {
    dark_cyan_ln!("\nEnter any character and press enter to view the answer:");
    let output :String = read!();
    if output == "minispielen" {
        let mut n = 0;
        while n < 69 {
            dark_green_ln!("Ping!");
            thread::sleep(time::Duration::from_millis(500));
            dark_red_ln!("Pong!");
            thread::sleep(time::Duration::from_millis(500));
            n += 1;
        }
    } else if output == "credits" {
        println!("This quiz has been developed by Job, enjoy!");
    } else {
        dark_cyan_ln!("The answer is:");
    };
}

fn questions(questionsvec : &mut Vec<i32>) -> Vec<i32>{
    let totalscore = questionsvec.len();
    let mut wrongquestions = vec![];
    let mut score :i32 = 0;

    while questionsvec.len() > 0 {
        let index = (rand::random::<f32>() * questionsvec.len() as f32).floor() as usize;
        let value = questionsvec.remove( index );
        //println!("{}", value);
        //println!("{:?}", questionsvec);

        if value == 1 {
            dark_cyan_ln!("Which two dimensions are considered to describe new-products?");
            let a = "Innovativeness & Newness to the firm";
            let b = "Newness to the firm & Newness to the market";
            let c = "Newness to the market & Innovativeness";
            let d = "Innovativeness & improvements";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 2 {
            dark_cyan_ln!("How can 'New to the world products' be described?");
            let a = "Newer to the firm but known to the market";
            let b = "Known to the firm and to the market";
            let c = "New to both the firm and the market";
            let d = "Known to the market but new for the firm";
            let e = "New to the market but known to the firm";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 3 {
            dark_cyan_ln!("How can 'Cost-reduction products' be described?");
            let a = "Newer to the firm but known to the market";
            let b = "Known to the firm and to the market";
            let c = "New to both the firm and the market";
            let d = "Known to the market but new for the firm";
            let e = "New to the market but known to the firm";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 4 {
            dark_cyan_ln!("How can 'Improvement products' be described?");
            let a = "Newer to the firm but known to the market";
            let b = "Known to the firm and to the market";
            let c = "New to both the firm and the market";
            let d = "Known to the market but new for the firm";
            let e = "New to the market but known to the firm";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 5 {
            dark_cyan_ln!("How can 'New product lines' be described?");
            let a = "Newer to the firm but known to the market";
            let b = "Known to the firm and to the market";
            let c = "New to both the firm and the market";
            let d = "Known to the market but new for the firm";
            let e = "New to the market but known to the firm";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 6 {
            dark_cyan_ln!("How can 'Repositionings' be described?");
            let a = "Newer to the firm but known to the market";
            let b = "Known to the firm and to the market";
            let c = "New to both the firm and the market";
            let d = "Known to the market but new for the firm";
            let e = "New to the market but known to the firm";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "E" {
                score += 1;
                dark_green_ln!("Correct: {}", e);
            } else if answer == "e" {
                score += 1;
                dark_green_ln!("Correct: {}", e);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", e);
                wrongquestions.push(value);
            }
        } else if value == 7 {
            dark_cyan_ln!("How much of the total profit do new products generate?");
            let a = "10-25%";
            let b = "25-50%";
            let c = "50-75%";
            let d = "75+%";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 8 {
            dark_cyan_ln!("The objective of innovation is adding value, this is done in two different ways:\n 
            (1) By internal learning and knowledge generation\n
            (2) By positive reception by customers and the market as a whole\n");
            let a = "1 is true, 2 is false";
            let b = "1 is false, 2 is true";
            let c = "1 and 2 are both true";
            let d = "1 and 2 are both false";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 9 {
            dark_cyan_ln!("Out of which 3 building blocks does a product idea consist?");
            let a = "Why: preference, what: concept, how: invention";
            let b = "Why: need, what: form, how: patents";
            let c = "Why: preference, prototype: form, how: R&D";
            let d = "Why: need, what: form/vague concept, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 10 {
            dark_cyan_ln!("The difference between an idea and an opportunity is that \nan opportunity is a precise specification of a customer problem,\n where an idea is a concept generated by using 2/3 building blocks.");
            let a = "True";
            let b = "False";
            //let c = "Why: preference, what: form, how: R&D";
            //let d = "Why: need, what: form, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 11 {
            dark_cyan_ln!("The three components of creativity are?");
            let a = "Expertise, creative thinking and motivation";
            let b = "Expertise, artistic, value";
            let c = "Skills, creative thinking, value";
            let d = "Skills, artistic, motivation";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 12 {
            dark_cyan_ln!("Managers can stimulate creativity by providing training, encouraging multifunctional cooperation, \nproviding challengs, giving freedom, and support / tolerance over mistakes");
            let a = "True";
            let b = "False";
            //let c = "Why: preference, what: form, how: R&D";
            //let d = "Why: need, what: form, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 13 {
            dark_cyan_ln!("Strategic product planning is focussing on gaps in the product portfolio, \ni.e. on the market, trend and technology. \n Resources are managed to ensure a steady stream of big new opportunities. \n Which tactic do they use?");
            let a = "Following demand, trying to stage a monopoly, maintaining the right balance";
            let b = "Producing a lot, value maximization, managing general direction";
            let c = "Value maximization, maintaining the right balance (low/high risk, long/short term), \nstrategic direction and amount of projects";
            let d = "Maintaining the right balance, value maximization, trying to stage a monopoly";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 14 {
            dark_cyan_ln!("A concepted is evaluated by using UNFISS, what does UNFISS stand for?");
            let a = "Uniqueness, Need fulfillment, Feasability, Impact, Scalability, Strategic fit";
            let b = "Uncertaintity, Need fulfillment, Fast progress, Impact, Scalability, Strategic fit";
            let c = "Uniqueness, Need fulfillment, Fear, Interest, Solvency, Strategic fit";
            let d = "Uniqueness, Need fulfillment, Feasability, Impact, Solvency, Strategic fit";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 15 {
            dark_cyan_ln!("What is used to evaluate, test and predict profits of a concept?");
            let a = "Evaluate: 5 Rogers variables, Test: UNFISS, Profit: ATAR";
            let b = "Evaluate: UNFISS, Test: 5 Rogers variables, Profit: ATAR";
            let c = "Evaluate: UNFISS, Test: ATAR, Profit: 5 Rogers variables";
            let d = "Evaluate: ATAR, Test: UNFISS, Profit: 5 Rogers variables";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 16 {
            dark_cyan_ln!("Are these the 5 variables of Rogers?\n1. Relative advantage,\n2. Required change,\n3. Ease of use,\n4. Triability by taste or experience (divisbility),\n5. The communicability and observability to see the benefits");
            let a = "True";
            let b = "False";
            //let c = "";
            //let d = "";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 17 {
            dark_cyan_ln!("ATAR (Awareness, Trial, Availability, Repeat) can be used to calculate \nthe amount of units sold using the formula:\nUnits available * % of customers aware of product * % of customers willing to pay \n * % availability at retailers * (1 + % repeated purchases * additional purchased units)");
            let a = "True";
            let b = "False";
            //let c = "Why: preference, what: form, how: R&D";
            //let d = "Why: need, what: form, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 18 {
            dark_cyan_ln!("The Ansoff matrix has the market on the vertical axis and technology on the horizontal axis.\nWhich are the 4 categories in this matrix?");
            let a = "Market development, production increase, market penetration, product development";
            let b = "Market development, product diversification, market penetration, product development";
            let c = "Market research, product diversification, market penetration, production increase";
            let d = "Market research, product diversification, market penetration, product development";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 19 {
            dark_cyan_ln!("Design For Manufacturability (DFM) is the process of\nproactively designing products to optimize manufacturing.\nWhich of these is NOT a guideline:");
            let a = "Adhere to specific process design guidelines";
            let b = "Avoid right/left hand parts";
            let c = "Design parts with symmetry";
            let d = "Customer research";
            let e = "If part symmetry is not possible, make parts very asymmetrical";
            let f = "Design for fixturing";
            let g = "Minimize tooling complexity by concurrently designing tooling";
            let h = "Specify optimal tolerance for a Robust Design";
            let i = "Specify quality parts from reliable sources";
            let j = "Minimize Setups";
            let k = "Minimize cutting tools";
            let l = "Understand tolerance step functions and specify tolerances wisely";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}\n(F) {}\n(G) {}\n(H) {}\n(I) {}\n(J) {}\n(K) {}\n(L) {}", a, b, c, d, e, f, g, h, i, j, k, l);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 20 {
            dark_cyan_ln!("Design for Aesthetics (DFA) exists out of 4 principles:\nMaximum effect for minimum means\nUnity in variety\nMost advanced, yet acceptable\nOptimal match");
            let a = "True";
            let b = "False";
            //let c = "Why: preference, what: form, how: R&D";
            //let d = "Why: need, what: form, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 21 {
            dark_cyan_ln!("Design for Serviceability (DFS) is the ability to diagnose, remove,\nreplace, adjust components with relative ease.");
            let a = "True";
            let b = "False";
            //let c = "Why: preference, what: form, how: R&D";
            //let d = "Why: need, what: form, how: relevant technology";
            dark_cyan_ln!("(A) {}\n(B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 22 {
            dark_cyan_ln!("Which order follows the V-model");
            let a = "implementation - concept ideas - operation & maintenance";
            let b = "concept ideas - operation & maintenance - implementation";
            let c = "operation & maintenance - concept ideas - implementation";
            let d = "Concept ideas - implementation - operation & maintenance";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 23 {
            dark_cyan_ln!("Modularity creates variability and upgradeability,\nwhen is a product considered modular?");
            anykey();

            dark_green_ln!("A product is modular when:\n
            1. Modules implement one or few functional elements (only a few functions depend on a single module).\n
            2. Couplings within a module are stronger than the couplings among modules.\n");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 24 {
            dark_cyan_ln!("TRIZ has as goal to promote free thinking.\n Which statement is NOT one of the 5 TRIZ Pillars?");
            let a = "Ideality: The weighting of costs and benefits against each other.";
            let b = "Profitability: The ability of the product to be as profitable as possible.";
            let c = "Contradictions: All systems have conflicting problems.";
            let d = "Functionality: Every system possesses a Main Useful Function (MUF),\nany system component which does not contribute towards the achievement\nof this function is ultimately harmful.";
            let e = "Space/time/interface: Looking at things from many different viewpoints.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 25 {
            dark_cyan_ln!("What is the difference between a Technical and Physical contradiction (TRIZ)?");
            anykey();

            dark_green_ln!("Technical is when improving one parameter hurts another parameter,\nPhysical is when 2 properties of the same parameter hurt each other.");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 26 {
            dark_cyan_ln!("What is NOT a separation principle of Physical contradictions (TRIZ)?");
            let a = "Separate in time: Having opposite things at different times";
            let b = "Separate by profitability: Wanting opposite things, but at high costs.";
            let c = "Separate in space: Wanting opposite things at the same time,\n but at different locations.";
            let d = "Separate on condition: Wanting opposite things at the same time,\n in the same place, but for different features.";
            let e = "Separate by system: Wanting opposite things at different levels.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 27 {
            dark_cyan_ln!("What is the downside of being first on the market?");
            anykey();

            dark_green_ln!("The market has no experience yet, making wrong estimations,\n and thus mistakes happens more frequently.");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 28 {
            dark_cyan_ln!("What are the two marketing launch decission types?");
            anykey();

            dark_green_ln!("Strategic launch (what, when, where, why) & Tactical launch (promotions, activities, pricing)");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 29 {
            dark_cyan_ln!("What is NOT an advantage of being first on the market?");
            let a = "Favourable reputation / Locking in customers";
            let b = "Having little regulations";
            let c = "Learning curve effects";
            let d = "Setting the standard for price, quality and service.";
            let e = "Patents and emptying scarce resources";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}\n(E) {}", a, b, c, d, e);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 30 {
            dark_cyan_ln!("What type of sale is pre-ordering");
            let a = "Pseudo sale: Speculative / simulated sales";
            let b = "Controlled sale: Informal sale direclty or to a mini market";
            let c = "Full sale: Rollout and test marketing";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}", a, b, c);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 31 {
            dark_cyan_ln!("Production ramp-up is done with a stepwise curve or a gradual curve.\nThere are 5 categories to consider for this, which one is missing:\n
            1. Plant and equipment\n
            2. Production planning and control\n
            3. Product design\n
            4. Organizatin and design\n
            5. ?\n");
            let answer :String = read!();
            if answer == "Workforce" {
                score += 1;
                dark_green_ln!("Correct: {}", answer);
            } else if answer == "workforce" {
                score += 1;
                dark_green_ln!("Correct: {}", answer);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: Workforce");
                wrongquestions.push(value);
            }
        } else if value == 32 {
            dark_cyan_ln!("What is tracking during launch management?");
            let a = "Following the project based on input data";
            let b = "A system to decide when to intervene to reduce failure";
            let c = "The ability to lay out the plans trajectory";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}", a, b, c);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 33 {
            dark_cyan_ln!("What is NOT a tracking feature?");
            let a = "Following the project based on input data against the plan";
            let b = "A system to decide when to intervene to reduce failure";
            let c = "The ability to lay out the plans trajectory";
            let d = "Selecting trigger points in the launch plan to kick in";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 34 {
            dark_cyan_ln!("What is the correct order of processes with a technical launch");
            let a = "Ramp-up => Testing => Learning";
            let b = "Testing => Ramp-up => Learning";
            let c = "Learning => Testing => Ramp-up";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}", a, b, c);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 35 {
            dark_cyan_ln!("Learning is important due to four trends, which trends?");
            anykey();

            dark_green_ln!("Learning trends:\n
            1. Rapid knowledge growth in many industries\n
            2. The shortening of product life cycles\n
            3. The increasing complexity of new ideas and technologies\n
            4. The competition forces you to stay ahead or catch up\n");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 36 {
            dark_cyan_ln!("Learning consists our of 4 elements, which elements?");
            anykey();

            dark_green_ln!("Learning elements:\n
            1. Focus on organisation\n
            2. Enhancing knowledge\n
            3. Facilitate changes in action\n
            4. Ongoing efor without a clear end-point\n");

            let a = "Had the correct thing in mind";
            let b = "Need to study harder";
            dark_cyan_ln!("(A) {}    (B) {}", a, b);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
            } else if answer == "a" {
                score += 1;
            } else {
                wrongquestions.push(value);
            }
        } else if value == 37 {
            dark_cyan_ln!("When comparing the results of the comparative performance assessment study by the\nProduct Development & Management Association from 2012 with the previous results\nfrom 2004 the following trend can be observed:");
            let a = "There is a trend away from formal development processes\nand toward more flexibility in product development. ";
            let b = "There is a trend toward using less formal processes\nin the front end of new product development. ";
            let c = "There is a trend toward conducting more formal portfolio\nreviews and using more portfolio management techniques.";
            let d = "There is a trend away from using and conducting many of\nthe available product development tools, techniques and practices.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 38 {
            dark_cyan_ln!("Which of the following product innovation types is newest to the market?");
            let a = "Repositionings";
            let b = "Cost reductions";
            let c = "Improvements of existing products";
            let d = "Line extensions";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 39 {
            dark_cyan_ln!("In class we have discussed the different objectives of new product development.\nBased on this discussion, in which situation should a company consider a new product\nthat got launched on the market as a true failure?");
            let a = "When the launched new product does not generate any profit after the forecasted\ntime to break-even has passed.";
            let b = "When the development and launch of the new product does not deliver any added value to the company.";
            let c = "When the market for the launched new product is already declining before the sales is taking off.";
            let d = "When the final new product does not deliver any added value to the customers when it is launched on the market.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 40 {
            dark_cyan_ln!("Having the right mix of high- versus low-risk projects in a company’s innovation funnel\nis part of having a good ________");
            let a = "Platform strategy";
            let b = "Launch strategy";
            let c = "Risk mitigation strategy";
            let d = "Portfolio strategy";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 41 {
            dark_cyan_ln!("Which of the following guidelines should a company keep in mind\nwhen (re-)designing its new product development process?");
            let a = "The process should be adjustable to different project types.";
            let b = "The process should consist of three general stages.";
            let c = "The process should be a tunnel not a funnel.";
            let d = "The process should be applicable in a step-by-step manner.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 42 {
            dark_cyan_ln!("Which of the following statements describes the discovery of a potentially successful new product idea?");
            let a = "A solution for an urgent customer problem.";
            let b = "An objective, unserved customer need.";
            let c = "An easily and inexpensive to develop new technology.";
            let d = "A hidden and unsolved customer problem.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 43 {
            dark_cyan_ln!("Why should companies encourage the use of electronic brainstorming\ninstead of traditional brainstorming for new product idea generation?");
            let a = "Electronic brainstorming better supports group think.";
            let b = "Electronic brainstorming inhibits the bazooka effect.";
            let c = "Electronic brainstorming prevents discussions of ideas.";
            let d = "Electronic brainstorming stimulates individual contribution.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 44 {
            dark_cyan_ln!("Which of the following customer research methods is most useful to identify\nthe motivations and values behind the problems or needs of customers? ");
            let a = "Complaint analysis";
            let b = "Conjoint analysis";
            let c = "Ethnography (research towards ethnecity  / social class)";
            let d = "Interviews";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 45 {
            dark_cyan_ln!("Which of the following managerial guidelines applies to all idea or\nconcept evaluations throughout the development process? ");
            let a = "The idea source should not be included in the evaluation committee.";
            let b = "The evaluations should be made in a purely rational manner.";
            let c = "The market potential of the resulting new product is the most important criterion.";
            let d = "The risk of making an evaluation mistake should be managed through mitigation.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 46 {
            dark_cyan_ln!("What is the major advantage of using role playing as a method to identify customer problems?");
            let a = "It is the least time consuming method and can be used when there is not much time for research.";
            let b = "The team can uncover problems that customers do not recognize themselves.";
            let c = "The team can easily communicate the findings by showing pictures and videos.";
            let d = "It is the most inexpensive method because no customers have to be paid for their participation.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 47 {
            red_ln!("A decay curve is a curve ranging from concept->launch on the horizontal axis,\nand number of ideas on the vertical axis.");
            dark_cyan_ln!("In a decay curve, a slow decay means that:");
            let a = "projects tend to be stopped later.";
            let b = "the evaluation gates are less 'fuzzy'";
            let c = "the evaluation gates have more 'teeth'";
            let d = "development costs are avoided";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 48 {
            dark_cyan_ln!("_______ is defined as the process a company employs to externally search for and source research,\nnew technologies, and products");
            let a = "T-P-M linkage";
            let b = "Open innovation";
            let c = "Creative abrasion";
            let d = "Leveraged creativity";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 49 {
            dark_cyan_ln!("Which of the following outcomes of a risk/payoff matrix, used for evaluating a new\nproduct process, is most likely to be the costliest for the firm? ");
            let a = "Stopping a project that is likely to fail";
            let b = "Continuing a project that is likely to fail";
            let c = "Stopping a project that is likely to succeed";
            let d = "Continuing a project that is likely to succeed";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 50 {
            dark_cyan_ln!("In product development, a ____ is defined as a set of systems and interfaces that form a\ncommon structure from which derivative products are created. ");
            let a = "Product architecture";
            let b = "Product strategy";
            let c = "Product platform";
            let d = "Product portfolio";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 51 {
            dark_cyan_ln!("In ATAR model the 'T' stands for");
            let a = "Test";
            let b = "Trial";
            let c = "Turn-over";
            let d = "Tactical";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 52 {
            dark_cyan_ln!("In which of the following organizational options does\nthe project manager have the least amount of power?");
            let a = "Project matrix option";
            let b = "Venture option";
            let c = "Balanced matrix option";
            let d = "Functional option";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 53 {
            dark_cyan_ln!("Which of the following statements is true of product design?");
            let a = "Product design is a multi-functional activity where all functions need to be involved.";
            let b = "The design of a new product should be considerd\nonly after the product architecture has been developed.";
            let c = "The design of a new product is the task of a industrial or product designers.";
            let d = "Firms that are judged to be a higher in design effectiveness\ntend to report lower profits due to higher R&D expenditure.";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 54 {
            dark_cyan_ln!("With reference to the design process, the practice of putting the various individuals\nor functional areas in close proximity as to shorten communication lines\nand increase team cohesion is called ______");
            let a = "relocation";
            let b = "resettlement";
            let c = "colocaton";
            let d = "approximation";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 55 {
            dark_cyan_ln!("The majority of the strategic launch decisions should be made:");
            let a = "At the beginning of the launch phase";
            let b = "before testing of the final product";
            let c = "during the front end phase";
            let d = "at the beginning of the development phase";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 56 {
            dark_cyan_ln!("The first 5-10% of customers who adopt a new product are referred to as ______");
            let a = "Innovators";
            let b = "Early adopters";
            let c = "Laggards";
            let d = "Early majority";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "A" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else if answer == "a" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", a);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", a);
                wrongquestions.push(value);
            }
        } else if value == 57 {
            dark_cyan_ln!("At which point in the launch cycle do the launch expenditures\ngenerally increate the most heavily?");
            let a = "During pre-launch";
            let b = "During beachhead";
            let c = "Before the announcement";
            let d = "During early growth";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "B" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else if answer == "b" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", b);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", b);
                wrongquestions.push(value);
            }
        } else if value == 58 {
            dark_cyan_ln!("When looking at the best performing companies in the comparative performance\nassesment study by the Product Development & Management Association from 2012\nthe following observation can be made:\nThe majority of the best performing companies have applied a ______");
            let a = "reactive strategy";
            let b = "niche strategy";
            let c = "fast follower strategy";
            let d = "first-to-the market strategy";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "D" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else if answer == "d" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", d);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", d);
                wrongquestions.push(value);
            }
        } else if value == 59 {
            dark_cyan_ln!("The trial phase in the adoption of a new product:");
            let a = "involves large scale usage of the product";
            let b = "occurs before the awareness phase";
            let c = "permits customers to verify claims about the product";
            let d = "involves no cost for both the firm and the customers";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 60 {
            dark_cyan_ln!("");
            let a = "";
            let b = "";
            let c = "";
            let d = "";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 61 {
            dark_cyan_ln!("");
            let a = "";
            let b = "";
            let c = "";
            let d = "";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
        } else if value == 62 {
            dark_cyan_ln!("");
            let a = "";
            let b = "";
            let c = "";
            let d = "";
            dark_cyan_ln!("(A) {}\n(B) {}\n(C) {}\n(D) {}", a, b, c, d);
            let answer :String = read!();
            if answer == "C" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else if answer == "c" {
                score += 1;
                dark_green_ln!("Presumably correct: {}", c);
            } else {
                dark_red_ln!("That answer was incorrect! \nThe presumably correct answer was: {}", c);
                wrongquestions.push(value);
            }
































        } else {
            red_ln!("This is not a question!");
        };





        let questionsleft = questionsvec.len(); //+ wrongquestions.len();
        if questionsleft > 1 {
            dark_yellow_ln!("There are {} questions left.\n", questionsleft);
        } else if questionsleft == 1 {
            dark_yellow_ln!("There is {} question left.\n", questionsleft);
        } else {
            dark_magenta_ln!("QUIZ COMPLETED!");
        };
        
    
        

    };









    

    dark_magenta_ln!("Your score is {} out of {}!!", score, totalscore);

    return wrongquestions;
}
