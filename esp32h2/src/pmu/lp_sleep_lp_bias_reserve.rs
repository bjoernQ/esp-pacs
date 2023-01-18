#[doc = "Register `LP_SLEEP_LP_BIAS_RESERVE` writer"]
pub struct W(crate::W<LP_SLEEP_LP_BIAS_RESERVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_SLEEP_LP_BIAS_RESERVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_SLEEP_LP_BIAS_RESERVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_SLEEP_LP_BIAS_RESERVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_SLEEP_LP_BIAS_RESERVE` writer - need_des"]
pub type LP_SLEEP_LP_BIAS_RESERVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LP_SLEEP_LP_BIAS_RESERVE_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_bias_reserve(&mut self) -> LP_SLEEP_LP_BIAS_RESERVE_W<0> {
        LP_SLEEP_LP_BIAS_RESERVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_sleep_lp_bias_reserve](index.html) module"]
pub struct LP_SLEEP_LP_BIAS_RESERVE_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lp_sleep_lp_bias_reserve::W](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_BIAS_RESERVE to value 0"]
impl crate::Resettable for LP_SLEEP_LP_BIAS_RESERVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}