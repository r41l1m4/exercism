pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    // starts with 1
    // from the second line on:
    // repeats the first, checks if there's more.
    // if there's more, sum the actual with the next.
    // if there's no more, repeats the last.
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![];

        for i in 0..self.row_count {
            if i == 0 { result.push(vec![1]); }
            else if i == 1 { result.push(vec![1, 1]); }
            else {
                let mut tmp = vec![1];
                let last_line = result.last().unwrap();

                for i in 1..last_line.len() {
                    let sum = last_line[i - 1] + last_line[i];
                    tmp.push(sum);
                }
                tmp.push(1);

                result.push(tmp)
            }
        }
        result
    }
}