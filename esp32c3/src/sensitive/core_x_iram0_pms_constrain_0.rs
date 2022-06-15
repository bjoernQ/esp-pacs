#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_0` reader"]
pub struct R(crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_PMS_CONSTRAIN_0` writer"]
pub struct W(crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_LOCK` reader - core_x_iram0_pms_constrain_lock"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `CORE_X_IRAM0_PMS_CONSTRAIN_LOCK` writer - core_x_iram0_pms_constrain_lock"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W<'a> =
    crate::BitWriter<'a, u32, CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - core_x_iram0_pms_constrain_lock"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_lock(&self) -> CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core_x_iram0_pms_constrain_lock"]
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_lock(&mut self) -> CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W {
        CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_pms_constrain_0](index.html) module"]
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_pms_constrain_0::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_pms_constrain_0::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_PMS_CONSTRAIN_0 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
