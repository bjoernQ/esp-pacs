#[doc = "Register `T_STRING` reader"]
pub type R = crate::R<T_STRING_SPEC>;
#[doc = "Register `T_STRING` writer"]
pub type W = crate::W<T_STRING_SPEC>;
#[doc = "Field `T_STRING` reader - Defines t_string for calculating the initial Hash value for SHA-512/t."]
pub type T_STRING_R = crate::FieldReader<u32>;
#[doc = "Field `T_STRING` writer - Defines t_string for calculating the initial Hash value for SHA-512/t."]
pub type T_STRING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    pub fn t_string(&self) -> T_STRING_R {
        T_STRING_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T_STRING")
            .field("t_string", &format_args!("{}", self.t_string().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T_STRING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    #[must_use]
    pub fn t_string(&mut self) -> T_STRING_W<T_STRING_SPEC, 0> {
        T_STRING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "String content register for calculating initial Hash Value (only effective for SHA-512/t)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_string::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_string::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_STRING_SPEC;
impl crate::RegisterSpec for T_STRING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_string::R`](R) reader structure"]
impl crate::Readable for T_STRING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_string::W`](W) writer structure"]
impl crate::Writable for T_STRING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T_STRING to value 0"]
impl crate::Resettable for T_STRING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
