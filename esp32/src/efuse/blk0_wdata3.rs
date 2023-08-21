#[doc = "Register `BLK0_WDATA3` reader"]
pub type R = crate::R<BLK0_WDATA3_SPEC>;
#[doc = "Register `BLK0_WDATA3` writer"]
pub type W = crate::W<BLK0_WDATA3_SPEC>;
#[doc = "Field `DISABLE_APP_CPU` reader - "]
pub type DISABLE_APP_CPU_R = crate::BitReader;
#[doc = "Field `DISABLE_BT` reader - "]
pub type DISABLE_BT_R = crate::BitReader;
#[doc = "Field `CHIP_PACKAGE_4BIT` reader - "]
pub type CHIP_PACKAGE_4BIT_R = crate::BitReader;
#[doc = "Field `DIS_CACHE` reader - "]
pub type DIS_CACHE_R = crate::BitReader;
#[doc = "Field `SPI_PAD_CONFIG_HD` reader - "]
pub type SPI_PAD_CONFIG_HD_R = crate::FieldReader;
#[doc = "Field `CHIP_PACKAGE` reader - "]
pub type CHIP_PACKAGE_R = crate::FieldReader;
#[doc = "Field `CHIP_PACKAGE` writer - "]
pub type CHIP_PACKAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CHIP_CPU_FREQ_LOW` reader - "]
pub type CHIP_CPU_FREQ_LOW_R = crate::BitReader;
#[doc = "Field `CHIP_CPU_FREQ_LOW` writer - "]
pub type CHIP_CPU_FREQ_LOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHIP_CPU_FREQ_RATED` reader - "]
pub type CHIP_CPU_FREQ_RATED_R = crate::BitReader;
#[doc = "Field `CHIP_CPU_FREQ_RATED` writer - "]
pub type CHIP_CPU_FREQ_RATED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLK3_PART_RESERVE` reader - "]
pub type BLK3_PART_RESERVE_R = crate::BitReader;
#[doc = "Field `BLK3_PART_RESERVE` writer - "]
pub type BLK3_PART_RESERVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHIP_VER_REV1` reader - "]
pub type CHIP_VER_REV1_R = crate::BitReader;
#[doc = "Field `CHIP_VER_REV1` writer - "]
pub type CHIP_VER_REV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESERVE_0_112` reader - "]
pub type RESERVE_0_112_R = crate::FieldReader<u16>;
#[doc = "Field `RESERVE_0_112` writer - "]
pub type RESERVE_0_112_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn disable_app_cpu(&self) -> DISABLE_APP_CPU_R {
        DISABLE_APP_CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn disable_bt(&self) -> DISABLE_BT_R {
        DISABLE_BT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chip_package_4bit(&self) -> CHIP_PACKAGE_4BIT_R {
        CHIP_PACKAGE_4BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dis_cache(&self) -> DIS_CACHE_R {
        DIS_CACHE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn spi_pad_config_hd(&self) -> SPI_PAD_CONFIG_HD_R {
        SPI_PAD_CONFIG_HD_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn chip_package(&self) -> CHIP_PACKAGE_R {
        CHIP_PACKAGE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn chip_cpu_freq_low(&self) -> CHIP_CPU_FREQ_LOW_R {
        CHIP_CPU_FREQ_LOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn chip_cpu_freq_rated(&self) -> CHIP_CPU_FREQ_RATED_R {
        CHIP_CPU_FREQ_RATED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn blk3_part_reserve(&self) -> BLK3_PART_RESERVE_R {
        BLK3_PART_RESERVE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chip_ver_rev1(&self) -> CHIP_VER_REV1_R {
        CHIP_VER_REV1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserve_0_112(&self) -> RESERVE_0_112_R {
        RESERVE_0_112_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA3")
            .field(
                "disable_app_cpu",
                &format_args!("{}", self.disable_app_cpu().bit()),
            )
            .field("disable_bt", &format_args!("{}", self.disable_bt().bit()))
            .field(
                "chip_package_4bit",
                &format_args!("{}", self.chip_package_4bit().bit()),
            )
            .field("dis_cache", &format_args!("{}", self.dis_cache().bit()))
            .field(
                "spi_pad_config_hd",
                &format_args!("{}", self.spi_pad_config_hd().bits()),
            )
            .field(
                "chip_package",
                &format_args!("{}", self.chip_package().bits()),
            )
            .field(
                "chip_cpu_freq_low",
                &format_args!("{}", self.chip_cpu_freq_low().bit()),
            )
            .field(
                "chip_cpu_freq_rated",
                &format_args!("{}", self.chip_cpu_freq_rated().bit()),
            )
            .field(
                "blk3_part_reserve",
                &format_args!("{}", self.blk3_part_reserve().bit()),
            )
            .field(
                "chip_ver_rev1",
                &format_args!("{}", self.chip_ver_rev1().bit()),
            )
            .field(
                "reserve_0_112",
                &format_args!("{}", self.reserve_0_112().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn chip_package(&mut self) -> CHIP_PACKAGE_W<BLK0_WDATA3_SPEC, 9> {
        CHIP_PACKAGE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn chip_cpu_freq_low(&mut self) -> CHIP_CPU_FREQ_LOW_W<BLK0_WDATA3_SPEC, 12> {
        CHIP_CPU_FREQ_LOW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn chip_cpu_freq_rated(&mut self) -> CHIP_CPU_FREQ_RATED_W<BLK0_WDATA3_SPEC, 13> {
        CHIP_CPU_FREQ_RATED_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn blk3_part_reserve(&mut self) -> BLK3_PART_RESERVE_W<BLK0_WDATA3_SPEC, 14> {
        BLK3_PART_RESERVE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn chip_ver_rev1(&mut self) -> CHIP_VER_REV1_W<BLK0_WDATA3_SPEC, 15> {
        CHIP_VER_REV1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserve_0_112(&mut self) -> RESERVE_0_112_W<BLK0_WDATA3_SPEC, 16> {
        RESERVE_0_112_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA3_SPEC;
impl crate::RegisterSpec for BLK0_WDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata3::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata3::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA3 to value 0"]
impl crate::Resettable for BLK0_WDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
