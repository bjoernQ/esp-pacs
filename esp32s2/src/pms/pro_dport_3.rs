#[doc = "Register `PRO_DPORT_3` reader"]
pub type R = crate::R<PRO_DPORT_3_SPEC>;
#[doc = "Register `PRO_DPORT_3` writer"]
pub type W = crate::W<PRO_DPORT_3_SPEC>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_1` reader - Configure read-protection address 1."]
pub type PRO_DPORT_RESERVE_FIFO_1_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_1` writer - Configure read-protection address 1."]
pub type PRO_DPORT_RESERVE_FIFO_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 1."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_1(&self) -> PRO_DPORT_RESERVE_FIFO_1_R {
        PRO_DPORT_RESERVE_FIFO_1_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_3")
            .field(
                "pro_dport_reserve_fifo_1",
                &format_args!("{}", self.pro_dport_reserve_fifo_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 1."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_1(&mut self) -> PRO_DPORT_RESERVE_FIFO_1_W<PRO_DPORT_3_SPEC, 0> {
        PRO_DPORT_RESERVE_FIFO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PeriBus1 permission control register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_3_SPEC;
impl crate::RegisterSpec for PRO_DPORT_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_3::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_3::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_3 to value 0"]
impl crate::Resettable for PRO_DPORT_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
