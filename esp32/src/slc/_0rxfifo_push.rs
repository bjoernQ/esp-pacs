#[doc = "Register `_0RXFIFO_PUSH` reader"]
pub type R = crate::R<_0RXFIFO_PUSH_SPEC>;
#[doc = "Register `_0RXFIFO_PUSH` writer"]
pub type W = crate::W<_0RXFIFO_PUSH_SPEC>;
#[doc = "Field `SLC0_RXFIFO_WDATA` reader - "]
pub type SLC0_RXFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RXFIFO_WDATA` writer - "]
pub type SLC0_RXFIFO_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `SLC0_RXFIFO_PUSH` reader - "]
pub type SLC0_RXFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `SLC0_RXFIFO_PUSH` writer - "]
pub type SLC0_RXFIFO_PUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc0_rxfifo_wdata(&self) -> SLC0_RXFIFO_WDATA_R {
        SLC0_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rxfifo_push(&self) -> SLC0_RXFIFO_PUSH_R {
        SLC0_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0RXFIFO_PUSH")
            .field(
                "slc0_rxfifo_wdata",
                &format_args!("{}", self.slc0_rxfifo_wdata().bits()),
            )
            .field(
                "slc0_rxfifo_push",
                &format_args!("{}", self.slc0_rxfifo_push().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0RXFIFO_PUSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxfifo_wdata(&mut self) -> SLC0_RXFIFO_WDATA_W<_0RXFIFO_PUSH_SPEC, 0> {
        SLC0_RXFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxfifo_push(&mut self) -> SLC0_RXFIFO_PUSH_W<_0RXFIFO_PUSH_SPEC, 16> {
        SLC0_RXFIFO_PUSH_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0rxfifo_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0rxfifo_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _0RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0rxfifo_push::R`](R) reader structure"]
impl crate::Readable for _0RXFIFO_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0rxfifo_push::W`](W) writer structure"]
impl crate::Writable for _0RXFIFO_PUSH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0RXFIFO_PUSH to value 0"]
impl crate::Resettable for _0RXFIFO_PUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
