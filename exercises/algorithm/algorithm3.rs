/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


// fn sort<T>(array: &mut [T]){
// 	//TODO
//     // 快速排序

// }

fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return; // 如果数组为空或只有一个元素，直接返回
    }
    let len = array.len();
    quick_sort(array, 0, len - 1);
}

fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(array, low, high);
        if pivot_index > 0 {
            quick_sort(array, low, pivot_index - 1); // 对左侧进行排序
        }
        quick_sort(array, pivot_index + 1, high);   // 对右侧进行排序
    }
}

fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;  // 选择最后一个元素作为 pivot
    let mut i = low;   // i 用来追踪比 pivot 小的元素的索引

    for j in low..high {
        if array[j] <= array[pivot] {
            array.swap(i, j);  // 如果当前元素比 pivot 小，交换它和 i 的元素
            i += 1;
        }
    }
    array.swap(i, pivot);  // 最后将 pivot 放到正确的位置
    i  // 返回 pivot 的最终位置
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}