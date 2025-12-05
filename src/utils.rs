#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub map: Vec<Vec<T>>,
    pub height: i32,
    pub width: i32
}
impl<T> Grid<T> {
    pub fn new(map: Vec<Vec<T>>) -> Self {
        Self { height:map.len() as i32, width:map[0].len() as i32, map }
    }
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(v) = self.map.get(y as usize) {
            v.get(x as usize)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(v) = self.map.get_mut(y as usize) {
            v.get_mut(x as usize)
        } else {
            None
        }
    }
    pub fn iter_flat(&self)-> impl Iterator<Item = (i32,i32)>{
        return (0..self.height).flat_map(|y| 
                                         (0..self.width).map(move |x, | (x,y)))
    }
}
