#[doc = "Register `RTCCALICFG1` reader"]
pub type R = crate::R<RTCCALICFG1_SPEC>;
#[doc = "Field `RTC_CALI_CYCLING_DATA_VLD` reader - Periodic calibration valid signal."]
pub type RTC_CALI_CYCLING_DATA_VLD_R = crate::BitReader;
#[doc = "Field `RTC_CALI_VALUE` reader - Calibration value when cycles of clock to be calibrated reach TIMG_RTC_CALI_MAX, in unit of XTAL_CLK clock cycles."]
pub type RTC_CALI_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Periodic calibration valid signal."]
    #[inline(always)]
    pub fn rtc_cali_cycling_data_vld(&self) -> RTC_CALI_CYCLING_DATA_VLD_R {
        RTC_CALI_CYCLING_DATA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 7:31 - Calibration value when cycles of clock to be calibrated reach TIMG_RTC_CALI_MAX, in unit of XTAL_CLK clock cycles."]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG1")
            .field(
                "rtc_cali_cycling_data_vld",
                &format_args!("{}", self.rtc_cali_cycling_data_vld().bit()),
            )
            .field(
                "rtc_cali_value",
                &format_args!("{}", self.rtc_cali_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTCCALICFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RTC calibration configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG1_SPEC;
impl crate::RegisterSpec for RTCCALICFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg1::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG1_SPEC {}
#[doc = "`reset()` method sets RTCCALICFG1 to value 0"]
impl crate::Resettable for RTCCALICFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
