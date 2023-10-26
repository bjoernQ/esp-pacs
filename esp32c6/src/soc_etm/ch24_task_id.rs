#[doc = "Register `CH24_TASK_ID` reader"]
pub type R = crate::R<CH24_TASK_ID_SPEC>;
#[doc = "Register `CH24_TASK_ID` writer"]
pub type W = crate::W<CH24_TASK_ID_SPEC>;
#[doc = "Field `CH24_TASK_ID` reader - ch24_task_id"]
pub type CH24_TASK_ID_R = crate::FieldReader;
#[doc = "Field `CH24_TASK_ID` writer - ch24_task_id"]
pub type CH24_TASK_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ch24_task_id"]
    #[inline(always)]
    pub fn ch24_task_id(&self) -> CH24_TASK_ID_R {
        CH24_TASK_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH24_TASK_ID")
            .field(
                "ch24_task_id",
                &format_args!("{}", self.ch24_task_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH24_TASK_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ch24_task_id"]
    #[inline(always)]
    #[must_use]
    pub fn ch24_task_id(&mut self) -> CH24_TASK_ID_W<CH24_TASK_ID_SPEC, 0> {
        CH24_TASK_ID_W::new(self)
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
#[doc = "channel24 task id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch24_task_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch24_task_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH24_TASK_ID_SPEC;
impl crate::RegisterSpec for CH24_TASK_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch24_task_id::R`](R) reader structure"]
impl crate::Readable for CH24_TASK_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch24_task_id::W`](W) writer structure"]
impl crate::Writable for CH24_TASK_ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH24_TASK_ID to value 0"]
impl crate::Resettable for CH24_TASK_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
