#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `REG_CLK_EN` reader - Reserved"]
pub type REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_CLK_EN` writer - Reserved"]
pub type REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_clk_en(&self) -> REG_CLK_EN_R {
        REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("reg_clk_en", &format_args!("{}", self.reg_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clk_en(&mut self) -> REG_CLK_EN_W<CLK_EN_SPEC> {
        REG_CLK_EN_W::new(self, 0)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0x01"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x01;
}