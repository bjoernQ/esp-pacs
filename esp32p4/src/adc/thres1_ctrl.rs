#[doc = "Register `THRES1_CTRL` reader"]
pub type R = crate::R<THRES1_CTRL_SPEC>;
#[doc = "Register `THRES1_CTRL` writer"]
pub type W = crate::W<THRES1_CTRL_SPEC>;
#[doc = "Field `THRES1_CHANNEL` reader - need_des"]
pub type THRES1_CHANNEL_R = crate::FieldReader;
#[doc = "Field `THRES1_CHANNEL` writer - need_des"]
pub type THRES1_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THRES1_HIGH` reader - saradc1's thres0 monitor thres"]
pub type THRES1_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `THRES1_HIGH` writer - saradc1's thres0 monitor thres"]
pub type THRES1_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `THRES1_LOW` reader - saradc1's thres0 monitor thres"]
pub type THRES1_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `THRES1_LOW` writer - saradc1's thres0 monitor thres"]
pub type THRES1_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn thres1_channel(&self) -> THRES1_CHANNEL_R {
        THRES1_CHANNEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:17 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    pub fn thres1_high(&self) -> THRES1_HIGH_R {
        THRES1_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 18:30 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    pub fn thres1_low(&self) -> THRES1_LOW_R {
        THRES1_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES1_CTRL")
            .field(
                "thres1_channel",
                &format_args!("{}", self.thres1_channel().bits()),
            )
            .field(
                "thres1_high",
                &format_args!("{}", self.thres1_high().bits()),
            )
            .field("thres1_low", &format_args!("{}", self.thres1_low().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES1_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_channel(&mut self) -> THRES1_CHANNEL_W<THRES1_CTRL_SPEC> {
        THRES1_CHANNEL_W::new(self, 0)
    }
    #[doc = "Bits 5:17 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<THRES1_CTRL_SPEC> {
        THRES1_HIGH_W::new(self, 5)
    }
    #[doc = "Bits 18:30 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_low(&mut self) -> THRES1_LOW_W<THRES1_CTRL_SPEC> {
        THRES1_LOW_W::new(self, 18)
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
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES1_CTRL_SPEC;
impl crate::RegisterSpec for THRES1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres1_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres1_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRES1_CTRL to value 0x0003_ffed"]
impl crate::Resettable for THRES1_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0003_ffed;
}