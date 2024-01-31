use std::fmt;

struct AppInfo {
    app_st_addr: u32,
    app_blk_count: u32,
    page_count: u32,
    pg_per_blk: Vec<u32>,
    offset: u32,
}

impl AppInfo {
    fn new(
        app_size: u32,
        st_addr_option: Option<u32>,
        p_count_option: Option<u32>,
        p_size_option: Option<u32>,
    ) -> Self {
        let st_addr = st_addr_option.unwrap_or(0);
        let p_count = p_count_option.unwrap_or(1024);
        let p_size = p_size_option.unwrap_or(4096);
        let app_st_addr = st_addr;
        let offset = if app_size % p_size == 0 {
            4095
        } else {
            app_size % p_size
        };

        let (app_blk_count, page_count, pg_per_blk) = if app_size < p_size {
            (1, 1, vec![1])
        } else if app_size < p_count * p_size {
            let page_count = (app_size / p_size) + 1;
            let tmp = p_count - st_addr / p_size;

            if page_count < tmp {
                (1, page_count, vec![page_count])
            } else {
                let app_blk_count = 2;
                let mut pg_per_blk = vec![0; app_blk_count as usize];

                pg_per_blk[0] = tmp;
                pg_per_blk[1] = page_count - tmp;

                (app_blk_count, page_count, pg_per_blk)
            }
        } else {
            let page_count = (app_size / p_size) + 1;
            let fblock = p_count - ((st_addr / p_size) % p_count);
            let lblock = (page_count - fblock) % p_count;
            let fullblocks = (page_count - fblock - lblock) / p_count;
            let mut size = fullblocks;

            if fblock != 0 {
                size += 1;
            }
            if lblock != 0 {
                size += 1;
            }

            let app_blk_count = size;
            let mut pg_per_blk = vec![0; size as usize];

            let mut i = 0;
            if fblock > 0 {
                pg_per_blk[0] = fblock;
                i += 1;
            }
            if lblock != 0 {
                pg_per_blk[size as usize - 1] = lblock;
            }
            for j in i..size as usize {
                pg_per_blk[j] = p_count;
            }

            (app_blk_count, page_count, pg_per_blk)
        };

        AppInfo {
            app_st_addr,
            app_blk_count,
            page_count,
            pg_per_blk,
            offset,
        }
    }

    fn next_address(&self, p_size_option: Option<u32>) -> u32 {
        let p_size = p_size_option.unwrap_or(4096_u32);
        self.app_st_addr + self.page_count * p_size
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n=========================================\n")?;
        write!(
            f,
            "The memory was allocated from the address: {}\n",
            self.app_st_addr
        )?;
        write!(f, "       Application Info : ")?;
        write!(f, "\n\t     Allocated Blocks : {}", self.app_blk_count)?;
        write!(f, "\n\t      Allocated Pages : {}", self.page_count)?;
        write!(f, "\n\t               Offset : {}", self.offset)?;
        write!(
            f,
            "\n\t         Last Address : {}",
            self.next_address(None) - 1
        )?;
        write!(f, "\n\t Pages in first Block : {}", self.pg_per_blk[0])?;
        write!(
            f,
            "\n\t Pages in  last Block : {}",
            self.pg_per_blk[self.app_blk_count as usize - 1]
        )?;
        if self.app_blk_count > 2 {
            write!(
                f,
                "\n\t  Fully allocated are : {} (Blocks)",
                self.app_blk_count - 2
            )?;
        }
        write!(
            f,
            "\n\tAllocated Memory size : {} (Bytes)\n",
            self.next_address(None) - self.app_st_addr
        )
    }
}

fn main() {
    #[warn(unused_mut)]
    let mut addr = 0;
    let app1 = AppInfo::new(1059800, Some(addr), None, None);
    println!("{}", app1);

    addr = app1.next_address(None);
    let app2 = AppInfo::new(5050097, Some(addr), None, None);
    println!("{}", app2);

    addr = app2.next_address(None);
    let app3 = AppInfo::new(41004305, Some(addr), None, None);
    println!("{}", app3);
}
