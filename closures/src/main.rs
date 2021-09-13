struct Lazy<T, V> where T: Fn(V) -> V, V: Copy {
    calculate: T,
    buff: Option<V>,
}

impl<T, V> Lazy<T, V> where T: Fn(V) -> V, V: Copy {
    fn new(calculate: T) -> Lazy<T, V> {
        Lazy {
            calculate,
            buff: None,
        }
    }

    fn value(&mut self, arg: V) -> V {
        return match self.buff {
            Some(x) => x,
            None => {
                println!("Calculation...");
                let result = (self.calculate)(arg);
                self.buff = Some(result);
                result
            }
        };
    }
}

fn main() {
    let cls1 = |x| x + 1;
    println!("{}", cls1(1));

    let parm = 2;
    let mut increment = Lazy::new(|x| x * x);
    println!("{}", increment.value(parm));
    println!("{}", increment.value(parm));


    let y = vec![1, 2, 3];
    let cls2 = move |x| x == y;

    println!("{}", cls2(vec![1, 2, 3]));

    // Will not compile cause y was moved.
    // println!("{:?}", y);

    let y = 10;
    let cls3 = move |x| x == y;

    println!("{}", cls3(10));

    // No errors cause i32 implements Copy trait
    println!("{}", y);


    let v = vec![1, 2, 3];
    let v: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v); // 2 3 4


    let v1: Vec<i32>  = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", v1);
    
    // Will not compile cause v moved to iterator
    //println!("{:?}", v);
}
