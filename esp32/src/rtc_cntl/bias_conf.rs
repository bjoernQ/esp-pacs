#[doc = "Register `BIAS_CONF` reader"]
pub type R = crate::R<BIAS_CONF_SPEC>;
#[doc = "Register `BIAS_CONF` writer"]
pub type W = crate::W<BIAS_CONF_SPEC>;
#[doc = "Field `DBG_ATTEN` reader - DBG_ATTEN"]
pub type DBG_ATTEN_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN` writer - DBG_ATTEN"]
pub type DBG_ATTEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ENB_SCK_XTAL` reader - ENB_SCK_XTAL"]
pub type ENB_SCK_XTAL_R = crate::BitReader;
#[doc = "Field `ENB_SCK_XTAL` writer - ENB_SCK_XTAL"]
pub type ENB_SCK_XTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INC_HEARTBEAT_REFRESH` reader - INC_HEARTBEAT_REFRESH"]
pub type INC_HEARTBEAT_REFRESH_R = crate::BitReader;
#[doc = "Field `INC_HEARTBEAT_REFRESH` writer - INC_HEARTBEAT_REFRESH"]
pub type INC_HEARTBEAT_REFRESH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEC_HEARTBEAT_PERIOD` reader - DEC_HEARTBEAT_PERIOD"]
pub type DEC_HEARTBEAT_PERIOD_R = crate::BitReader;
#[doc = "Field `DEC_HEARTBEAT_PERIOD` writer - DEC_HEARTBEAT_PERIOD"]
pub type DEC_HEARTBEAT_PERIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INC_HEARTBEAT_PERIOD` reader - INC_HEARTBEAT_PERIOD"]
pub type INC_HEARTBEAT_PERIOD_R = crate::BitReader;
#[doc = "Field `INC_HEARTBEAT_PERIOD` writer - INC_HEARTBEAT_PERIOD"]
pub type INC_HEARTBEAT_PERIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEC_HEARTBEAT_WIDTH` reader - DEC_HEARTBEAT_WIDTH"]
pub type DEC_HEARTBEAT_WIDTH_R = crate::BitReader;
#[doc = "Field `DEC_HEARTBEAT_WIDTH` writer - DEC_HEARTBEAT_WIDTH"]
pub type DEC_HEARTBEAT_WIDTH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST_BIAS_I2C` reader - RST_BIAS_I2C"]
pub type RST_BIAS_I2C_R = crate::BitReader;
#[doc = "Field `RST_BIAS_I2C` writer - RST_BIAS_I2C"]
pub type RST_BIAS_I2C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    pub fn dbg_atten(&self) -> DBG_ATTEN_R {
        DBG_ATTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&self) -> ENB_SCK_XTAL_R {
        ENB_SCK_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&self) -> INC_HEARTBEAT_REFRESH_R {
        INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&self) -> DEC_HEARTBEAT_PERIOD_R {
        DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&self) -> INC_HEARTBEAT_PERIOD_R {
        INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&self) -> DEC_HEARTBEAT_WIDTH_R {
        DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RST_BIAS_I2C"]
    #[inline(always)]
    pub fn rst_bias_i2c(&self) -> RST_BIAS_I2C_R {
        RST_BIAS_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIAS_CONF")
            .field("dbg_atten", &format_args!("{}", self.dbg_atten().bits()))
            .field(
                "enb_sck_xtal",
                &format_args!("{}", self.enb_sck_xtal().bit()),
            )
            .field(
                "inc_heartbeat_refresh",
                &format_args!("{}", self.inc_heartbeat_refresh().bit()),
            )
            .field(
                "dec_heartbeat_period",
                &format_args!("{}", self.dec_heartbeat_period().bit()),
            )
            .field(
                "inc_heartbeat_period",
                &format_args!("{}", self.inc_heartbeat_period().bit()),
            )
            .field(
                "dec_heartbeat_width",
                &format_args!("{}", self.dec_heartbeat_width().bit()),
            )
            .field(
                "rst_bias_i2c",
                &format_args!("{}", self.rst_bias_i2c().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BIAS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 24:25 - DBG_ATTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten(&mut self) -> DBG_ATTEN_W<BIAS_CONF_SPEC, 24> {
        DBG_ATTEN_W::new(self)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn enb_sck_xtal(&mut self) -> ENB_SCK_XTAL_W<BIAS_CONF_SPEC, 26> {
        ENB_SCK_XTAL_W::new(self)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_refresh(&mut self) -> INC_HEARTBEAT_REFRESH_W<BIAS_CONF_SPEC, 27> {
        INC_HEARTBEAT_REFRESH_W::new(self)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_period(&mut self) -> DEC_HEARTBEAT_PERIOD_W<BIAS_CONF_SPEC, 28> {
        DEC_HEARTBEAT_PERIOD_W::new(self)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_period(&mut self) -> INC_HEARTBEAT_PERIOD_W<BIAS_CONF_SPEC, 29> {
        INC_HEARTBEAT_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_width(&mut self) -> DEC_HEARTBEAT_WIDTH_W<BIAS_CONF_SPEC, 30> {
        DEC_HEARTBEAT_WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - RST_BIAS_I2C"]
    #[inline(always)]
    #[must_use]
    pub fn rst_bias_i2c(&mut self) -> RST_BIAS_I2C_W<BIAS_CONF_SPEC, 31> {
        RST_BIAS_I2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bias_conf::R`](R) reader structure"]
impl crate::Readable for BIAS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bias_conf::W`](W) writer structure"]
impl crate::Writable for BIAS_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIAS_CONF to value 0"]
impl crate::Resettable for BIAS_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
