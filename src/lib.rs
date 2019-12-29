#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl ANARGS {
    pub fn new() -> Self {
        //mimic atsa-nogui.c
        let mut args: ANARGS = unsafe { std::mem::zeroed() };
        args.start = ATSA_START as f32;
        args.duration = ATSA_DUR as f32;
        args.lowest_freq = ATSA_LFREQ as f32;
        args.highest_freq = ATSA_HFREQ as f32;
        args.freq_dev = ATSA_FREQDEV as f32;
        args.win_cycles = ATSA_WCYCLES as std::os::raw::c_int;
        args.win_type = ATSA_WTYPE as std::os::raw::c_int;
        args.hop_size = ATSA_HSIZE as f32;
        args.lowest_mag = ATSA_LMAG as f32;
        args.track_len = ATSA_TRKLEN as std::os::raw::c_int;
        args.min_seg_len = ATSA_MSEGLEN as std::os::raw::c_int;
        args.min_gap_len = ATSA_MGAPLEN as std::os::raw::c_int;
        args.SMR_thres = ATSA_SMRTHRES as f32;
        args.min_seg_SMR = ATSA_MSEGSMR as f32;
        args.last_peak_cont = ATSA_LPKCONT as f32;
        args.SMR_cont = ATSA_SMRCONT as f32;
        args.type_ = ATSA_TYPE as std::os::raw::c_int;
        args
    }
}

impl Default for ANARGS {
    fn default() -> Self {
        Self::new()
    }
}
