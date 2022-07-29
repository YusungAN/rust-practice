use num::traits::Num;
use num::ToPrimitive;

#[derive(Debug)]
struct Mat<T> {
    elements: Vec<Vec<T>>,
    row: usize,
    col: usize
}

impl<T: Num+Copy+Default+ToPrimitive> Mat<T> {
    fn is_vaild(&self) {
        if self.row != self.elements.len() {
            panic!("invalid matrix");
        }
        for i in &self.elements {
            if i.len() != self.col {
                panic!("invalid matrix");
            }
        }
    }

    fn dot(&self, mat: &Mat<T>) -> Mat<T> { // 곱
        self.is_vaild();
        let mut result = Vec::new();
        if self.col != mat.row {
            panic!("invalid matrix shape");
        }

        for i in 0..self.row {
            let mut temp_vec = Vec::new();
            for j in 0..mat.col {
                let mut temp = T::default();
                for k in 0..self.col {
                    temp = temp + self.elements[i][k] * mat.elements[k][j];
                }
                temp_vec.push(temp);
            }
            result.push(temp_vec);
        }

        Mat { elements: result, row: self.row, col: mat.col }
    }

    fn transpose(&self) -> Mat<T> { // 전치행렬
        let mut res = Vec::new();

        for i in 0..self.col {
            let mut v = Vec::new();
            for j in 0..self.row {
                v.push(self.elements[j][i]);
            }
            res.push(v);
        }

        Mat { elements: res, row: self.col, col: self.row }
    }

    fn mean(&self) -> f64 { // 평균
        let mut sum = T::default();
        for i in 0..self.row {
            for j in 0..self.col {
                sum = sum + self.elements[i][j];
            }
        }

        sum.to_f64().unwrap() / (self.col * self.row).to_f64().unwrap()
    }


}
fn main() {
    let m1 = Mat {
        elements: vec![vec![1, 2, 3], vec![3, 4, 5]],
        row: 2,
        col: 3
    };

    let m2 = Mat {
        elements: vec![vec![1, 2], vec![3, 4], vec![5, 6]],
        row: 3, 
        col: 2
    };

    println!("{:?}", m1.dot(&m2));
    println!("{:?}", m1.transpose());
    println!("{}", m2.mean());

}
