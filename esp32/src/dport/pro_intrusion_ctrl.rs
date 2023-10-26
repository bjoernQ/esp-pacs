#[doc = "Register `PRO_INTRUSION_CTRL` reader"]
pub type R = crate::R<PRO_INTRUSION_CTRL_SPEC>;
#[doc = "Register `PRO_INTRUSION_CTRL` writer"]
pub type W = crate::W<PRO_INTRUSION_CTRL_SPEC>;
#[doc = "Field `PRO_INTRUSION_RECORD_RESET_N` reader - "]
pub type PRO_INTRUSION_RECORD_RESET_N_R = crate::BitReader;
#[doc = "Field `PRO_INTRUSION_RECORD_RESET_N` writer - "]
pub type PRO_INTRUSION_RECORD_RESET_N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_intrusion_record_reset_n(&self) -> PRO_INTRUSION_RECORD_RESET_N_R {
        PRO_INTRUSION_RECORD_RESET_N_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTRUSION_CTRL")
            .field(
                "pro_intrusion_record_reset_n",
                &format_args!("{}", self.pro_intrusion_record_reset_n().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_INTRUSION_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_intrusion_record_reset_n(
        &mut self,
    ) -> PRO_INTRUSION_RECORD_RESET_N_W<PRO_INTRUSION_CTRL_SPEC, 0> {
        PRO_INTRUSION_RECORD_RESET_N_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intrusion_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_intrusion_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTRUSION_CTRL_SPEC;
impl crate::RegisterSpec for PRO_INTRUSION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intrusion_ctrl::R`](R) reader structure"]
impl crate::Readable for PRO_INTRUSION_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_intrusion_ctrl::W`](W) writer structure"]
impl crate::Writable for PRO_INTRUSION_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_INTRUSION_CTRL to value 0x01"]
impl crate::Resettable for PRO_INTRUSION_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
