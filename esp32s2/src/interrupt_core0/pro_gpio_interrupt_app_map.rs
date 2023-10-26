#[doc = "Register `PRO_GPIO_INTERRUPT_APP_MAP` reader"]
pub type R = crate::R<PRO_GPIO_INTERRUPT_APP_MAP_SPEC>;
#[doc = "Register `PRO_GPIO_INTERRUPT_APP_MAP` writer"]
pub type W = crate::W<PRO_GPIO_INTERRUPT_APP_MAP_SPEC>;
#[doc = "Field `PRO_GPIO_INTERRUPT_APP_MAP` reader - This register is used to map GPIO_INTERRUPT_APP interrupt signal to one of the CPU interrupts."]
pub type PRO_GPIO_INTERRUPT_APP_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_GPIO_INTERRUPT_APP_MAP` writer - This register is used to map GPIO_INTERRUPT_APP interrupt signal to one of the CPU interrupts."]
pub type PRO_GPIO_INTERRUPT_APP_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to map GPIO_INTERRUPT_APP interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    pub fn pro_gpio_interrupt_app_map(&self) -> PRO_GPIO_INTERRUPT_APP_MAP_R {
        PRO_GPIO_INTERRUPT_APP_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_GPIO_INTERRUPT_APP_MAP")
            .field(
                "pro_gpio_interrupt_app_map",
                &format_args!("{}", self.pro_gpio_interrupt_app_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_GPIO_INTERRUPT_APP_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to map GPIO_INTERRUPT_APP interrupt signal to one of the CPU interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pro_gpio_interrupt_app_map(
        &mut self,
    ) -> PRO_GPIO_INTERRUPT_APP_MAP_W<PRO_GPIO_INTERRUPT_APP_MAP_SPEC, 0> {
        PRO_GPIO_INTERRUPT_APP_MAP_W::new(self)
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
#[doc = "GPIO_INTERRUPT_APP interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_app_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_app_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_GPIO_INTERRUPT_APP_MAP_SPEC;
impl crate::RegisterSpec for PRO_GPIO_INTERRUPT_APP_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_gpio_interrupt_app_map::R`](R) reader structure"]
impl crate::Readable for PRO_GPIO_INTERRUPT_APP_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_gpio_interrupt_app_map::W`](W) writer structure"]
impl crate::Writable for PRO_GPIO_INTERRUPT_APP_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_GPIO_INTERRUPT_APP_MAP to value 0x10"]
impl crate::Resettable for PRO_GPIO_INTERRUPT_APP_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
