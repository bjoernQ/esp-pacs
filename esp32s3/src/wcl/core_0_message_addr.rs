#[doc = "Register `Core_0_MESSAGE_ADDR` reader"]
pub type R = crate::R<CORE_0_MESSAGE_ADDR_SPEC>;
#[doc = "Register `Core_0_MESSAGE_ADDR` writer"]
pub type W = crate::W<CORE_0_MESSAGE_ADDR_SPEC>;
#[doc = "Field `CORE_0_MESSAGE_ADDR` reader - This field is used to set address that need to write when enter WORLD0"]
pub type CORE_0_MESSAGE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_MESSAGE_ADDR` writer - This field is used to set address that need to write when enter WORLD0"]
pub type CORE_0_MESSAGE_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This field is used to set address that need to write when enter WORLD0"]
    #[inline(always)]
    pub fn core_0_message_addr(&self) -> CORE_0_MESSAGE_ADDR_R {
        CORE_0_MESSAGE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_MESSAGE_ADDR")
            .field(
                "core_0_message_addr",
                &format_args!("{}", self.core_0_message_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_MESSAGE_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field is used to set address that need to write when enter WORLD0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_message_addr(&mut self) -> CORE_0_MESSAGE_ADDR_W<CORE_0_MESSAGE_ADDR_SPEC, 0> {
        CORE_0_MESSAGE_ADDR_W::new(self)
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
#[doc = "Clear writer_buffer write address configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_message_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_message_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_MESSAGE_ADDR_SPEC;
impl crate::RegisterSpec for CORE_0_MESSAGE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_message_addr::R`](R) reader structure"]
impl crate::Readable for CORE_0_MESSAGE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_message_addr::W`](W) writer structure"]
impl crate::Writable for CORE_0_MESSAGE_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_MESSAGE_ADDR to value 0"]
impl crate::Resettable for CORE_0_MESSAGE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
