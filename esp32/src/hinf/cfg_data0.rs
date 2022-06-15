#[doc = "Register `CFG_DATA0` reader"]
pub struct R(crate::R<CFG_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA0` writer"]
pub struct W(crate::W<CFG_DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA0_SPEC>;
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
impl From<crate::W<CFG_DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER_ID_FN1` reader - "]
pub type USER_ID_FN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USER_ID_FN1` writer - "]
pub type USER_ID_FN1_W<'a> = crate::FieldWriter<'a, u32, CFG_DATA0_SPEC, u16, u16, 16, 0>;
#[doc = "Field `DEVICE_ID_FN1` reader - "]
pub type DEVICE_ID_FN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVICE_ID_FN1` writer - "]
pub type DEVICE_ID_FN1_W<'a> = crate::FieldWriter<'a, u32, CFG_DATA0_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn1(&self) -> USER_ID_FN1_R {
        USER_ID_FN1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn1(&self) -> DEVICE_ID_FN1_R {
        DEVICE_ID_FN1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn1(&mut self) -> USER_ID_FN1_W {
        USER_ID_FN1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn1(&mut self) -> DEVICE_ID_FN1_W {
        DEVICE_ID_FN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data0](index.html) module"]
pub struct CFG_DATA0_SPEC;
impl crate::RegisterSpec for CFG_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data0::R](R) reader structure"]
impl crate::Readable for CFG_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data0::W](W) writer structure"]
impl crate::Writable for CFG_DATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_DATA0 to value 0x2222_6666"]
impl crate::Resettable for CFG_DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2222_6666
    }
}
