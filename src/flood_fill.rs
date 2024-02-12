/**
 * An image is represented by an m x n integer grid image where image[i][j] represents the pixel
 * value of the image.
 *
 * You are also given three integers sr, sc, and color. You should prform a flood fill on the image
 * starting from the pixel image[sr][sc].
 *
 * To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally
 * to the starting pixel of the same color as the starting pixel, plus any pixels connected
 * 4-directionally to those pixels (also with the same color), and so on. Replace the color of all
 * of the aforementioned pixels with color.
 */
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![vec![false;image[0].len()];image.len()];
    let mut result = image;
    let i:usize = match sr.try_into() {
        Ok(value) => value,
        _ => {         
            print!("hi3");
            return result;
        },
    };
    let j:usize = match sc.try_into() {
        Ok(value) => value,
        _ => {         
            print!("hi3");
            return result;
        },
    };
    let t_color = result[i][j];
    back(&mut result, sr, sc, color, t_color, &mut visited);
    result
}

fn back(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, t_color: i32, visited: &mut Vec<Vec<bool>>) -> () {
    let i:usize = match sr.try_into() {
        Ok(value) => value,
        _ => {         
            return;
        },
    };
    let j:usize = match sc.try_into() {
        Ok(value) => value,
        _ => {         
            return;
        },
    };
    
    if i >= image.len() || i < 0 || j >= image[i].len() || j < 0 || visited[i][j] {
        print!("hi2");
        return;
    }
    print!("hi\n");
    let old_color = image[i][j];
    if t_color == old_color {
        image[i][j] = color;
        visited[i][j] = true;
        back(image,sr+1,sc, color, t_color, visited);
        back(image,sr,sc+1, color, t_color, visited);
        back(image,sr-1,sc, color, t_color, visited);
        back(image,sr,sc-1, color, t_color, visited);
    }
}
