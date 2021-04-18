const SIZE: usize = 1024;

/// Does some silly calculations just to keep the CPU busy.
pub fn waste_cpu_cycles() {
    let mut input = [0usize; SIZE];
    let mut output = [0usize; SIZE];
    for (i, v) in input.iter_mut().enumerate() {
        *v = i;
    }
    for v in &input {
        let w = input.iter().map(|x| v * x).reduce(|a, b| a + b).unwrap();
        output.iter_mut().for_each(|x| *x += w);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
