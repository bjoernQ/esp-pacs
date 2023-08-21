#[doc = "Register `REG_MAP1` reader"]
pub type R = crate::R<REG_MAP1_SPEC>;
#[doc = "Register `REG_MAP1` writer"]
pub type W = crate::W<REG_MAP1_SPEC>;
#[doc = "Field `MAP1` reader - x"]
pub type MAP1_R = crate::FieldReader<u32>;
#[doc = "Field `MAP1` writer - x"]
pub type MAP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map1(&self) -> MAP1_R {
        MAP1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP1")
            .field("map1", &format_args!("{}", self.map1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_MAP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn map1(&mut self) -> MAP1_W<REG_MAP1_SPEC, 0> {
        MAP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP1_SPEC;
impl crate::RegisterSpec for REG_MAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map1::R`](R) reader structure"]
impl crate::Readable for REG_MAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map1::W`](W) writer structure"]
impl crate::Writable for REG_MAP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_MAP1 to value 0"]
impl crate::Resettable for REG_MAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
