#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `THRES1_LOW_INT_ENA` reader - interrupt of thres1 low"]
pub type THRES1_LOW_INT_ENA_R = crate::BitReader;
#[doc = "Field `THRES1_LOW_INT_ENA` writer - interrupt of thres1 low"]
pub type THRES1_LOW_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES0_LOW_INT_ENA` reader - interrupt of thres0 low"]
pub type THRES0_LOW_INT_ENA_R = crate::BitReader;
#[doc = "Field `THRES0_LOW_INT_ENA` writer - interrupt of thres0 low"]
pub type THRES0_LOW_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES1_HIGH_INT_ENA` reader - interrupt of thres1 high"]
pub type THRES1_HIGH_INT_ENA_R = crate::BitReader;
#[doc = "Field `THRES1_HIGH_INT_ENA` writer - interrupt of thres1 high"]
pub type THRES1_HIGH_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRES0_HIGH_INT_ENA` reader - interrupt of thres0 high"]
pub type THRES0_HIGH_INT_ENA_R = crate::BitReader;
#[doc = "Field `THRES0_HIGH_INT_ENA` writer - interrupt of thres0 high"]
pub type THRES0_HIGH_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` reader - interrupt of sar2 done"]
pub type APB_SARADC2_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` writer - interrupt of sar2 done"]
pub type APB_SARADC2_DONE_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` reader - interrupt of sar1 done"]
pub type APB_SARADC1_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` writer - interrupt of sar1 done"]
pub type APB_SARADC1_DONE_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    pub fn thres1_low_int_ena(&self) -> THRES1_LOW_INT_ENA_R {
        THRES1_LOW_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    pub fn thres0_low_int_ena(&self) -> THRES0_LOW_INT_ENA_R {
        THRES0_LOW_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    pub fn thres1_high_int_ena(&self) -> THRES1_HIGH_INT_ENA_R {
        THRES1_HIGH_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    pub fn thres0_high_int_ena(&self) -> THRES0_HIGH_INT_ENA_R {
        THRES0_HIGH_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&self) -> APB_SARADC2_DONE_INT_ENA_R {
        APB_SARADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&self) -> APB_SARADC1_DONE_INT_ENA_R {
        APB_SARADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "thres1_low_int_ena",
                &format_args!("{}", self.thres1_low_int_ena().bit()),
            )
            .field(
                "thres0_low_int_ena",
                &format_args!("{}", self.thres0_low_int_ena().bit()),
            )
            .field(
                "thres1_high_int_ena",
                &format_args!("{}", self.thres1_high_int_ena().bit()),
            )
            .field(
                "thres0_high_int_ena",
                &format_args!("{}", self.thres0_high_int_ena().bit()),
            )
            .field(
                "apb_saradc2_done_int_ena",
                &format_args!("{}", self.apb_saradc2_done_int_ena().bit()),
            )
            .field(
                "apb_saradc1_done_int_ena",
                &format_args!("{}", self.apb_saradc1_done_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_low_int_ena(&mut self) -> THRES1_LOW_INT_ENA_W<INT_ENA_SPEC, 26> {
        THRES1_LOW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_low_int_ena(&mut self) -> THRES0_LOW_INT_ENA_W<INT_ENA_SPEC, 27> {
        THRES0_LOW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_high_int_ena(&mut self) -> THRES1_HIGH_INT_ENA_W<INT_ENA_SPEC, 28> {
        THRES1_HIGH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_high_int_ena(&mut self) -> THRES0_HIGH_INT_ENA_W<INT_ENA_SPEC, 29> {
        THRES0_HIGH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc2_done_int_ena(&mut self) -> APB_SARADC2_DONE_INT_ENA_W<INT_ENA_SPEC, 30> {
        APB_SARADC2_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc1_done_int_ena(&mut self) -> APB_SARADC1_DONE_INT_ENA_W<INT_ENA_SPEC, 31> {
        APB_SARADC1_DONE_INT_ENA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
