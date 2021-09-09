// struct FileHeader {
//     file_page_offset: usize, // 整个表中的第几个页
//     file_page_next: usize,  // 下一个页的位置
// }
// impl FileHeader {
//     pub fn new(file_page_offset: usize, file_page_next: usize) -> Self {
//         FileHeader{file_page_offset, file_page_next}
//     }
// }

// struct PageHeader {
//     page_heap_top: usize,  // 首个数据的位置,
//     page_n_heap: usize,  // 堆中的记录数,
//     page_last_insert: usize, // 最后插入的位置,
//     page_leval: usize, // 表示当前页在索引树的位置, 就是第几层, 0表示叶节点, 向上递增,
//     page_index_id: usize, // 索引id, 表示当前页属于那个索引, (好像是指father)
// }
// impl PageHeader {
//     pub fn new(page_heap_top: usize, page_n_heap: usize, page_last_insert: usize, page_leval: usize, page_index_id: usize,) -> Self {
//         PageHeader { page_heap_top, page_n_heap, page_last_insert, page_leval, page_index_id }
//     }
// }

// pub struct BranchNode {
//     fileheader: FileHeader,
//     page_header: PageHeader,
//     ids: Vec<>
// }

// pub struct Builder {

//     root:

// }
// impl Builder {
//     pub fn new(file_name: &str) -> Self {

//     }
// }
