#[doc = "Register `ENABLE1` reader"]
pub type R = crate::R<ENABLE1_SPEC>;
#[doc = "Register `ENABLE1` writer"]
pub type W = crate::W<ENABLE1_SPEC>;
#[doc = "Field `DATA` reader - GPIO output enable register for GPIO32-34"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - GPIO output enable register for GPIO32-34"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GPIO output enable register for GPIO32-34"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE1")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO output enable register for GPIO32-34"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<ENABLE1_SPEC, 0> {
        DATA_W::new(self)
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
#[doc = "GPIO output enable register for GPIO32-34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_SPEC;
impl crate::RegisterSpec for ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable1::R`](R) reader structure"]
impl crate::Readable for ENABLE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable1::W`](W) writer structure"]
impl crate::Writable for ENABLE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE1 to value 0"]
impl crate::Resettable for ENABLE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
